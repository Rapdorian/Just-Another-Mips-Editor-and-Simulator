use std::collections::HashMap;

mod instruction;
mod opcode;

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
    Comment(String),
    Blank,
}

/// Stores labels
#[derive(Default, Debug)]
pub struct LabelTable {
    labels: HashMap<String, u32>,

    // Is kept sorted by PC value
    lines: Vec<(usize, u32)>,
}

impl LabelTable {
    /// Insert a textual label
    pub fn insert_label(&mut self, key: String, v: u32) {
        self.labels.insert(key, v);
    }

    /// Insert a source line
    ///
    /// The key is the source line
    ///
    /// the value is the PC
    pub fn insert_line(&mut self, key: usize, v: u32) {
        self.lines.push((key, v));

        // sort the lines by the PC to assist looking up source code lines from a PC
        self.lines.sort_by_key(|x| x.1);
    }

    pub fn get_label(&self, key: &str) -> Option<u32> {
        dbg!(self.labels.get(key).map(|x| *x))
    }

    /// Gets the source code line for a given PC
    pub fn get_line(&self, pc: u32) -> Option<usize> {
        if dbg!(dbg!(pc) < dbg!(TEXT_BASE)) {
            return None;
        }

        // since self.lines is sorted by PC we can use a binary sort a return the closest value
        let idx = match dbg!(self.lines.binary_search_by_key(&pc, |x| x.1)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        dbg!(self.lines.get(idx)).map(|x| x.0)
    }
}
