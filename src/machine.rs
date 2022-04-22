use std::ops::ControlFlow;

use crate::{
    parser::{
        self, compute_labels,
        model::{LabelTable, Line, Segment, Segments, STACK_BASE, TEXT_BASE},
    },
    pipeline::{self, PipelineState},
    syscall::{resolve_syscall, Syscall},
    Memory, Register, RegisterFile, SP,
};
use anyhow::Result;

/// Represents an instance of a simulated MIPS computer.
#[derive(Default)]
pub struct Machine {
    pc: u32,
    regs: RegisterFile,
    state: PipelineState,
    mem: Memory,
    syms: LabelTable,
    pending_syscall: Option<Syscall>,
}

impl Machine {
    /// Fetch a readonly view of this machines registers
    pub fn register(&self, reg: Register) -> u32 {
        self.regs.read_register(reg)
    }

    pub fn register_mut(&mut self, reg: Register) -> &mut u32 {
        self.regs.get_mut(reg)
    }

    pub fn read_word(&self, addr: u32) -> Result<u32> {
        self.mem.get(addr)
    }

    pub fn write_word(&mut self, addr: u32, val: u32) {
        *self.mem.get_mut(addr) = val;
    }

    /// Reset this machine so it can be ran again
    ///
    /// Note that this will not reset the contents of memory or registers for that see
    /// [`hard_reset`]
    pub fn reset(&mut self) {
        self.pc = TEXT_BASE;
        self.state = PipelineState::default();
        self.regs = RegisterFile::default();
    }

    /// Fully resets this machine including memory contents and registers
    pub fn hard_reset(&mut self) {
        self.mem = Memory::default();
        self.syms = LabelTable::default();
        self.reset();
    }

    /// Set the contents of this machines memory to `mem`
    pub fn flash(&mut self, mem: Memory, syms: LabelTable) {
        self.mem = mem;
        self.syms = syms;
    }

    /// Gets the current source code line
    pub fn current_line(&mut self) -> [Option<usize>; 5] {
        [
            self.syms.get_line(self.state.if_id.pc),
            self.syms.get_line(self.state.id_ex.pc),
            self.syms.get_line(self.state.ex_mem.pc),
            self.syms.get_line(self.state.mem_wb.pc),
            self.syms.get_line(self.state.pipe_out.pc),
        ]
    }

    /// Gets the current pipeline stages
    pub fn pipeline(&self) -> &PipelineState {
        &self.state
    }

    /// Get the current contents of the stack
    pub fn stack(&mut self) -> Vec<(u32, u32)> {
        let sp = self.regs.read_register(SP) / 4;
        let mut stack = vec![];
        for i in sp..STACK_BASE / 4 {
            let addr = i * 4;
            stack.push((addr, self.mem.get(addr).unwrap_or(0)));
        }
        stack
    }

    pub fn resolve_input(&mut self, input: &str) -> Result<()> {
        if let Some(syscall) = &self.pending_syscall {
            resolve_syscall(&mut self.regs, syscall, input)?;
            self.pending_syscall = None;
        }
        Ok(())
    }

    /// Handle a syscall in the application
    ///
    /// # Returns
    /// If the syscall has been fully handled in the closure it should return `ControlFlow::Break`
    /// IF the syscall needs to be handled at a later time then return `ControlFlow::Continue`
    pub fn handle_syscall<F>(&mut self, f: F)
    where
        F: FnOnce(&Syscall) -> ControlFlow<()>,
    {
        // if there is a pending syscall try to handle it
        if let Some(syscall) = &self.pending_syscall {
            if let ControlFlow::Break(_) = (f)(syscall) {
                self.pending_syscall = None;
            }
        }
    }

    /// Checks if there is a pending syscall
    ///
    /// # Returns
    /// True if there is a pending syscall
    pub fn pending_syscall(&self) -> bool {
        self.pending_syscall.is_some()
    }

    /// Step the machine forward 1 cpu cycle
    pub fn cycle(&mut self) -> Result<()> {
        // do not cycle if we are waiting on a syscall
        if self.pending_syscall.is_none() {
            let (new_state, syscall) = pipeline::pipe_cycle(
                &mut self.pc,
                &mut self.regs,
                &mut self.mem,
                self.state.clone(),
            )?;
            self.state = new_state;
            if let Some(syscall) = syscall {
                self.pending_syscall = Some(syscall);
            }
        }
        Ok(())
    }
}

/// Method that create a memory instance from a script file
pub fn assembler(script: &str) -> Result<(Memory, LabelTable)> {
    // parse assembly
    let lines = parser::parse_string(script)?;
    let labels = compute_labels(&lines);

    // for each line in the parsed assembly assemble that line and add the result to a vec
    let mut memory = Memory::new();
    let mut segments = Segments::default();
    // current segement pc
    let mut pc = segments.switch(Segment::Text);
    for line in &lines {
        match line {
            Line::Instruction(ins) => {
                for word in ins {
                    let (bin, _) = word.asm(&labels, *pc);
                    //println!("{pc:X} {bin:X}\t{word:?}");
                    for byte in bin {
                        memory.set_byte(*pc, byte)?;
                        *pc += 1;
                    }
                }
            }
            Line::Segment(seg) => pc = segments.switch(*seg),
            _ => {}
        }
    }
    // insert guard instruction that causes the program to crash if it is encountered
    pc = segments.switch(Segment::Text);
    *memory.get_mut(*pc) = 0xBAADF00D;

    Ok((memory, labels))
}
