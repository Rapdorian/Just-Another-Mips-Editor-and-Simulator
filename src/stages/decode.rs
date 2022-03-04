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
    let j_mask = 0b00000011111111111111111111111111;
    let imm_mask = fn_mask | sh_mask | rd_mask;

    // Use masks to get the field values
    let rd = (input.instruction & rd_mask) >> 11;
    let rt = (input.instruction & rt_mask) >> 16;
    let rs = (input.instruction & rs_mask) >> 21;
    let funct = input.instruction & fn_mask;
    let shamt = (input.instruction & sh_mask) >> 6;
    let op = (input.instruction & op_mask) >> 26;
    let mut imm = input.instruction & imm_mask;
    let j_imm = input.instruction & j_mask;

    // make registers typed
    let rs: Register = rs.into();
    let rt: Register = rt.into();
    let rd: Register = rd.into();

    // read rs and rt
    let read_rs = reg_file.read_register(rs);
    let read_rt = reg_file.read_register(rt);

    // handle controls
    let reg_dst; // determines destination register (0: rt, 1: rd)
    let alu_src; // if enabled use immediate value as alu arg2
    let mem_to_reg; // if enabled dest register gets a memory location otheriwse gets alu result
    let reg_write; // if disabled don't write to dest register
    let mem_write; // if enabled write to alu result
    let mem_read; // if enabled read from alu result
    let alu_op; // alu operation
    let branch; // enable branching
    let jump; // enable jumping
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
            jump = false;
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
            jump = false;
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
            jump = false;
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
            jump = false;
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
            jump = false;
            alu_op = OP_AND;
        }

        0xf => {
            // LUI instruction
            reg_dst = false;
            alu_src = true;
            mem_to_reg = false;
            reg_write = true;
            mem_write = false;
            mem_read = false;
            alu_op = OP_UPPER;
            branch = false;
            jump = false;
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
            jump = false;
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
            jump = false;
            alu_op = OP_SUB;
        }
        0x02 => {
            // J instruction
            reg_dst = false;
            alu_src = false;
            mem_to_reg = false;
            reg_write = false;
            mem_read = false;
            mem_write = false;
            branch = false;
            jump = true;
            alu_op = OP_ADD;
            imm = j_imm;
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
        jump,
        pc: input.pc,
        syscall,
        instruction: input.instruction,
    }
}
