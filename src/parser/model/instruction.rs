use std::ops::{BitAnd, Shl};

use super::{LabelTable, Opcode};
use crate::Register;

#[derive(Debug)]
pub enum Symbol {
    Label(String),
    Address(u32),
}

impl Symbol {
    pub fn asm(&self, labels: &LabelTable) -> u32 {
        (match self {
            Symbol::Label(ref name) => labels.get_label(name).unwrap_or(0),
            Symbol::Address(x) => *x,
        } & 0x0FFFFFFF)
            >> 2
    }
}

#[derive(Debug)]
pub enum Imm {
    Label(String),
    HighHWord(String),
    LowHWord(String),
    Value(i64),
    PcRelative(String),
}

impl Imm {
    pub fn asm(&self, labels: &LabelTable, pc: u32) -> u32 {
        match self {
            Imm::Label(ref name) => labels.get_label(name).unwrap_or(0),
            Imm::HighHWord(ref name) => (labels.get_label(name).unwrap_or(0) & 0xFFFF0000) >> 16,
            Imm::LowHWord(ref name) => labels.get_label(name).unwrap_or(0) & 0xFFFF,
            Imm::Value(x) => *x as u32,
            Imm::PcRelative(ref name) => {
                let offset = (labels.get_label(name).unwrap_or(0)).wrapping_sub(pc + 4 as u32) >> 2;
                offset
            }
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    R {
        op: Opcode,
        rd: Register,
        rs: Register,
        rt: Register,
        shamt: u32,
    },
    I {
        op: Opcode,
        rt: Register,
        rs: Register,
        imm: Imm,
    },
    J {
        op: Opcode,
        addr: Symbol,
    },
    Literal {
        data: u32,
    },
}

fn field(x: u32, start: u32, width: u32) -> u32 {
    (x & (2_u32.pow(width) - 1)) << start
}

impl Instruction {
    pub fn asm(&self, labels: &LabelTable, pc: u32) -> u32 {
        match self {
            Instruction::R {
                op,
                rd,
                rs,
                rt,
                shamt,
            } => {
                field(op.value(), 0, 6)
                    | field(rd.value(), 11, 6)
                    | field(rt.value(), 16, 6)
                    | field(rs.value(), 21, 6)
                    | field(*shamt, 6, 5)
            }
            Instruction::I { op, rt, rs, imm } => {
                field(op.value(), 26, 6)
                    | field(imm.asm(labels, pc), 0, 16)
                    | field(rt.value(), 16, 5)
                    | field(rs.value(), 21, 5)
            }
            Instruction::Literal { data } => *data,
            Instruction::J { op, addr } => {
                field(op.value(), 26, 6) | field(addr.asm(labels), 0, 26)
            }
        }
    }
}
