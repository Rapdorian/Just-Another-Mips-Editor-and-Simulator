use std::collections::HashMap;

mod instruction;
mod opcode;

pub use super::ParseError;
pub use instruction::*;
pub use opcode::Opcode;

#[derive(Debug, Copy, Clone)]
pub enum Segment {
    Text,
    Data,
}

pub const TEXT_BASE: u32 = 0x00400000;
pub const DATA_BASE: u32 = 0x10010000;
pub const STACK_BASE: u32 = 0x7fffeffc;

/// Tracks the current position in each segment
pub struct Segments {
    segments: Vec<u32>,
}

impl Default for Segments {
    fn default() -> Self {
        Self {
            segments: vec![TEXT_BASE, DATA_BASE],
        }
    }
}

impl Segments {
    pub fn switch(&mut self, seg: Segment) -> &mut u32 {
        match seg {
            Segment::Text => &mut self.segments[0],
            Segment::Data => &mut self.segments[1],
        }
    }
}

#[derive(Debug)]
pub enum Line {
    Instruction(Vec<Instruction>),
    Label(String),
    Segment(Segment),
}

pub type LabelTable = HashMap<String, u32>;
