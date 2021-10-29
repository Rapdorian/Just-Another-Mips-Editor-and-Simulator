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
    pub shamt: u32,
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
    let alu_ctrl: (bool, bool, u8);
    if input.alu_op & 0b11 == 0b10 {
        // get info from instruction funct
        alu_ctrl = match input.op_funct {
            0x20 => (false, false, ALU_ADD), // add
            0x22 => (false, true, ALU_ADD),  // sub
            0x24 => (false, false, ALU_AND), // and
            0x2a => (false, true, ALU_SLT),  // slt
            0x25 => (false, false, ALU_OR),  // or
            0x27 => (true, true, ALU_AND),   // nor
            0x00 => (false, false, ALU_SLL), // sll
            0x02 => (false, false, ALU_SRL), // srl
            0x03 => (false, false, ALU_SRA), // sra
            _ => {
                panic!("Unkown Instruction")
            }
        };
    } else if input.alu_op & 0b11 == 0b00 {
        alu_ctrl = (false, false, ALU_ADD)
    } else {
        alu_ctrl = (false, false, ALU_ADD);
    }

    // Handle ALU operation
    let mut arg1 = input.reg_1;
    let mut arg2 = if input.alu_src {
        input.imm
    } else {
        input.reg_2
    };

    // check if we are using a shift operation.
    // and load the shamt if so
    match alu_ctrl.2 {
        ALU_SLL | ALU_SRL | ALU_SRA => {
            arg1 = arg2;
            arg2 = input.shamt;
        }
        _ => {}
    }

    let result = alu(arg1, arg2, alu_ctrl);
    println!("{} {} = {}", arg1, arg2, result);

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
const ALU_AND: u8 = 0;
const ALU_OR: u8 = 1;
const ALU_ADD: u8 = 2;
const ALU_SLT: u8 = 3;
const ALU_SLL: u8 = 4;
const ALU_SRL: u8 = 5;
const ALU_SRA: u8 = 6;

/// Simple ALU implementation.
/// TODO: Handle carry flag
pub fn alu(a: u32, b: u32, op: (bool, bool, u8)) -> u32 {
    let a = if op.0 { !a } else { a };
    let b = if op.1 { !b } else { b };

    // this is a hack since we haven't implemented carry bits yet
    let arith_a = if op.0 { a + 1 } else { a };
    let arith_b = if op.1 { b + 1 } else { b };

    match op.2 {
        ALU_AND => a & b,
        ALU_OR => a | b,
        ALU_ADD => arith_a.overflowing_add(arith_b).0,
        ALU_SLL => a.overflowing_shl(b).0,

        // Rust uses signedness to select between logical and arithmetic right shifts
        ALU_SRL => a.overflowing_shr(b).0,
        ALU_SRA => (a as i32).overflowing_shr(b).0 as u32,

        ALU_SLT => {
            if a < b {
                1
            } else {
                0
            }
        }
        _ => todo!("Unknown instruction: {:?}", op),
    }
}
