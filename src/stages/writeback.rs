use crate::{Register, RegisterFile, A0, V0};
use std::io;

/// struct representing this structs input
#[derive(Default, Clone)]
pub struct MemWb {
    pub mem_to_reg: bool,
    pub mem_data: u32,
    pub alu_data: u32,
    pub write_register: Register,
    pub reg_write: bool,
    pub syscall: bool,
}

pub struct PipelineOutput {
    pub syscall: bool,
}

pub fn writeback(reg_file: &mut RegisterFile, input: MemWb) -> PipelineOutput {
    if input.reg_write {
        if input.mem_to_reg {
            reg_file.write_register(input.write_register, input.mem_data);
        } else {
            reg_file.write_register(input.write_register, input.alu_data);
        }
    }
    PipelineOutput {
        syscall: input.syscall,
    }
}
