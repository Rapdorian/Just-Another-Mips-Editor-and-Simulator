use super::writeback::MemWb;
use crate::{Memory, Register};

/// Struct representing this stages input
#[derive(Debug, Default, Clone)]
pub struct ExMem {
    // stage data
    pub alu_result: u32,
    pub zero: bool,
    pub branch: bool,
    pub jump: bool,
    pub write_data: u32,
    pub write: bool,
    pub read: bool,
    pub branch_pc: u32,
    pub jump_pc: u32,
    // forwarded data
    pub mem_to_reg: bool,
    pub write_register: Register,
    pub reg_write: bool,
    pub syscall: bool,

    // demo thing
    pub instruction: u32,
}

/// Memory access pipeline stage
pub fn memory(pc: &mut u32, memory: &mut Memory, input: ExMem) -> MemWb {
    let mut read_data = 0;

    // handle memory accesses
    if input.write {
        *memory.get_mut(input.alu_result) = input.write_data;
    }
    if input.read {
        read_data = memory.get(input.alu_result);
    }

    if input.branch && input.zero {
        // branch to PC copmuted in execute stage
        *pc = input.branch_pc;
    }

    if input.jump {
        *pc = input.jump_pc;
    }

    MemWb {
        mem_to_reg: input.mem_to_reg,
        mem_data: read_data,
        alu_data: input.alu_result,
        write_register: input.write_register,
        reg_write: input.reg_write,
        syscall: input.syscall,
        instruction: input.instruction,
    }
}
