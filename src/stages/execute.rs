use crate::Register;

use super::memory::ExMem;

/// Struct representing this stages input
pub struct IdEx {
    // stage data
    pub alu_src: bool,
    pub reg_dst: bool,
    pub alu_op: u8,
    pub op_funct: u8,
    pub reg_1: u32,
    pub reg_2: u32,
    pub imm: u32,
    pub rt: Register,
    pub rd: Register,
    // forwarded data
    pub branch: bool,
    pub pc: u32,
    pub mem_write: bool,
    pub mem_read: bool,
    pub mem_to_reg: bool,
    pub reg_write: bool,
}

/// Runs execute stage
pub fn execute(input: IdEx) -> ExMem {
    // compute ALU control lines
    let alu_ctrl: u8;
    if input.alu_op & 0b11 == 0b10 {
        // get info from instruction funct
        let f0 = input.op_funct & 0b1;
        let f1 = (input.op_funct & 0b10) >> 1;
        let f2 = (input.op_funct & 0b100) >> 2;
        let f3 = (input.op_funct & 0b1000) >> 3;

        // TODO: simplify this into a match
        let ctrl0 = (f0 | f3) & 1;
        let ctrl1 = (!f2) & 1;
        let ctrl2 = (f1) & 1;

        alu_ctrl = ctrl0 | (ctrl1 << 1) | (ctrl2 << 2);
    } else if input.alu_op & 0b11 == 0b00 {
        alu_ctrl = 0b10;
    } else {
        alu_ctrl = 0b110;
    }

    // Handle ALU operation
    let arg1 = input.reg_1;
    let arg2 = if input.alu_src {
        input.imm
    } else {
        input.reg_2
    };

    let result = alu(arg1, arg2, alu_ctrl);

    ExMem {
        alu_result: result,
        zero: result == 0,
        write_data: input.reg_2,
        write: input.mem_write,
        read: input.mem_read,
        mem_to_reg: input.mem_to_reg,
        write_register: if input.reg_dst { input.rd } else { input.rt },
        reg_write: input.reg_write,
        branch: input.branch,
        branch_pc: input.pc + (input.imm), //TODO: Once address are fixed this needs to left shifted
    }
}

/// Simple ALU implementation.
pub fn alu(a: u32, b: u32, op: u8) -> u32 {
    match op {
        0b000 => a & b,
        0b001 => a | b,
        0b010 => a + b,
        0b110 => a - b,
        0b111 => {
            if a < b {
                1
            } else {
                0
            }
        }
        _ => todo!("Unknown instruction: {:b}", op),
    }
}
