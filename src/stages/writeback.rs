use crate::{Register, RegisterFile};

/// struct representing this structs input
pub struct MemWb {
    pub mem_to_reg: bool,
    pub mem_data: u32,
    pub alu_data: u32,
    pub write_register: Register,
    pub reg_write: bool,
}

pub fn writeback(reg_file: &mut RegisterFile, input: MemWb) {
    if input.reg_write {
        if input.mem_to_reg {
            reg_file.write_register(input.write_register, input.mem_data);
        } else {
            reg_file.write_register(input.write_register, input.alu_data);
        }
    }
}
