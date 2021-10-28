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
        alu_ctrl = match input.op_funct {
            0x20 => ALU_ADD,
            0x22 => ALU_SUB,
            0x24 => ALU_AND,
            0x2a => ALU_SLT,
            0x25 => ALU_OR,
            _ => {
                panic!("Unkown Instruction")
            }
        };
    } else if input.alu_op & 0b11 == 0b00 {
        alu_ctrl = ALU_ADD;
    } else {
        alu_ctrl = ALU_SUB;
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
        branch_pc: input.pc + (input.imm << 2),
    }
}

/// ALU Controls
const ALU_AND: u8 = 0b000;
const ALU_OR: u8 = 0b001;
const ALU_ADD: u8 = 0b010;
const ALU_SUB: u8 = 0b110;
const ALU_SLT: u8 = 0b111;

/// Simple ALU implementation.
pub fn alu(a: u32, b: u32, op: u8) -> u32 {
    match op {
        ALU_AND => a & b,
        ALU_OR => a | b,
        ALU_ADD => a + b,
        ALU_SUB => a - b,
        ALU_SLT => {
            if a < b {
                1
            } else {
                0
            }
        }
        _ => todo!("Unknown instruction: {:b}", op),
    }
}
