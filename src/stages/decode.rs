use crate::{
    stages::execute::{op_ctrl::*, IdEx},
    Register, RegisterFile,
};

// Struct representing this stages inputs
#[derive(Debug, Default, Clone)]
pub struct IfId {
    pub instruction: u32,
    pub pc: u32,
}

/// Decodes and instruction
pub fn decode(reg_file: &mut RegisterFile, input: IfId) -> IdEx {
    // instruction masks
    let fn_mask = 0b00000000000000000000000000111111;
    let sh_mask = 0b00000000000000000000011111000000;
    let rd_mask = 0b00000000000000001111100000000000;
    let rt_mask = 0b00000000000111110000000000000000;
    let rs_mask = 0b00000011111000000000000000000000;
    let op_mask = 0b11111100000000000000000000000000;
    let imm_mask = fn_mask | sh_mask | rd_mask;

    // Use masks to get the field values
    let rd = (input.instruction & rd_mask) >> 11;
    let rt = (input.instruction & rt_mask) >> 16;
    let rs = (input.instruction & rs_mask) >> 21;
    let funct = input.instruction & fn_mask;
    let shamt = (input.instruction & sh_mask) >> 6;
    let op = (input.instruction & op_mask) >> 26;
    let imm = input.instruction & imm_mask;

    // make registers typed
    let rs: Register = rs.into();
    let rt: Register = rt.into();
    let rd: Register = rd.into();

    // read rs and rt
    let read_rs = reg_file.read_register(rs);
    let read_rt = reg_file.read_register(rt);

    // handle controls
    let reg_dst;
    let alu_src;
    let mem_to_reg;
    let reg_write;
    let mem_write;
    let mem_read;
    let alu_op;
    let branch;
    let mut syscall = false;

    // This is where instructions are defined
    match op {
        0 => {
            syscall = funct == 0x0c;
            // R-type instruction
            reg_dst = true;
            alu_src = false;
            mem_to_reg = false;
            reg_write = true;
            mem_read = false;
            mem_write = false;
            branch = false;
            alu_op = OP_R;
        }
        0x23 => {
            // LW instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = true;
            reg_write = true;
            mem_read = true;
            mem_write = false;
            branch = false;
            alu_op = OP_ADD;
        }
        0x2b => {
            // SW instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = false;
            mem_read = false;
            mem_write = true;
            branch = false;
            alu_op = OP_ADD;
        }
        0x8 => {
            // ADDI instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = true;
            mem_read = false;
            mem_write = false;
            branch = false;
            alu_op = OP_ADD;
        }

        0xc => {
            // ANDI instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = true;
            mem_read = false;
            mem_write = false;
            branch = false;
            alu_op = OP_AND;
        }

        0xd => {
            // ORI instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = true;
            mem_read = false;
            mem_write = false;
            branch = false;
            alu_op = OP_OR;
        }
        0x4 => {
            // BEQ instruction
            reg_dst = false;
            alu_src = false;
            mem_to_reg = false;
            reg_write = false;
            mem_read = false;
            mem_write = false;
            branch = true;
            alu_op = OP_SUB;
        }
        _ => {
            todo!("implement missing instruction: 0x{:x}", op)
        }
    }

    IdEx {
        alu_src,
        reg_dst,
        alu_op,
        op_funct: funct as u8,
        reg_1: read_rs,
        reg_2: read_rt,
        imm,
        shamt,
        rt,
        rs,
        rd,
        mem_write,
        mem_read,
        mem_to_reg,
        reg_write,
        branch,
        pc: input.pc,
        syscall,
        instruction: input.instruction,
    }
}
