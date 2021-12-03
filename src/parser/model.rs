use std::collections::HashMap;

mod instruction;
mod opcode;

pub use super::ParseError;
pub use instruction::*;
pub use opcode::Opcode;

#[derive(Debug)]
pub enum Line {
    Instruction(Vec<Instruction>),
    Label(String),
}

pub type LabelTable = HashMap<String, usize>;
