use std::ops::ControlFlow;

use crate::{
    parser::{self, compute_labels, model::Line},
    pipeline::{self, PipelineState},
    syscall::{resolve_syscall, Syscall},
    Memory, Register, RegisterFile,
};
use anyhow::Result;

/// Represents an instance of a simulated MIPS computer.
#[derive(Default)]
pub struct Machine {
    pc: u32,
    regs: RegisterFile,
    state: PipelineState,
    mem: Memory,
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

    pub fn read_word(&self, addr: u32) -> u32 {
        self.mem.read_word(addr).unwrap_or(0)
    }

    pub fn write_word(&mut self, addr: u32, val: u32) {
        self.mem.write_word(addr, val).unwrap_or(()); // TODO: This is a bad idea
    }

    /// Reset this machine so it can be ran again
    ///
    /// Note that this will not reset the contents of memory or registers for that see
    /// [`hard_reset`]
    pub fn reset(&mut self) {
        self.pc = 0;
        self.state = PipelineState::default();
    }

    /// Fully resets this machine including memory contents and registers
    pub fn hard_reset(&mut self) {
        self.mem = Memory::default();
        self.regs = RegisterFile::default();
        self.reset();
    }

    /// Set the contents of this machines memory to `mem`
    pub fn flash(&mut self, mem: Memory) {
        self.mem = mem;
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

    /// Step the machine forward 1 cpu cycle
    pub fn cycle(&mut self) {
        // do not cycle if we are waiting on a syscall
        if self.pending_syscall.is_none() {
            let (new_state, syscall) = pipeline::pipe_cycle(
                &mut self.pc,
                &mut self.regs,
                &mut self.mem,
                self.state.clone(),
            );
            self.state = new_state;
            if let Some(syscall) = syscall {
                self.pending_syscall = Some(syscall);
            }
        }
    }
}

pub fn assembler(script: &str) -> Result<Memory> {
    // parse assembly
    let lines = parser::parse_string(script)?;
    let labels = compute_labels(&lines);

    // for each line in the parsed assembly assemble that line and add the result to a vec
    let mut raw_mem = vec![];
    let mut assembler_pc = 0;
    for line in &lines {
        match line {
            Line::Instruction(ins) => {
                for word in ins {
                    raw_mem.push(word.asm(&labels, assembler_pc));
                    assembler_pc += 4;
                }
            }
            Line::Label(_) => {}
        }
    }

    // create our memory object
    Ok(Memory::from_word_vec(raw_mem))
}
