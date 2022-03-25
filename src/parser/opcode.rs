use super::directives::{ascii_lit, asciiz_lit, segment, word_lit};
use super::instruction::{
    branch_type, i_type, j_type, jr_type, li_ins, load_type, lui, move_ins, multi_branch, nop,
    r_type, shift_type, syscall,
};
use super::model::{Line, Opcode, Segment};

use nom::combinator::fail;
use nom::error::{context, VerboseError};
use nom::{bytes::complete::take_till, combinator::map_res, IResult};

type InsParser = fn(&str, Opcode) -> IResult<&str, Line, VerboseError<&str>>;

const NO_PARSER: InsParser = |input, _| context("No parser for instruction", fail)(input);

/// Holds a parsed opcode and a nom parser that can parse its arguments and produce an Instruction
/// object
pub struct InstructionParser {
    op: Opcode,
    parser: Box<dyn Fn(&str, Opcode) -> IResult<&str, Line, VerboseError<&str>>>,
}

impl InstructionParser {
    /// Create a normal parser that needs to know its own opcode
    pub fn new<F>(op: Opcode, parser: F) -> Self
    where
        F: Fn(&str, Opcode) -> IResult<&str, Line, VerboseError<&str>> + 'static,
    {
        Self {
            op,
            parser: Box::new(parser),
        }
    }

    /// Creates a pseudo instruction parser that doesn't need to know its own opcode
    pub fn pseudo<F>(parser: F) -> Self
    where
        F: Fn(&str) -> IResult<&str, Line, VerboseError<&str>> + 'static,
    {
        Self {
            op: Opcode::Op(0),
            parser: Box::new(move |i, _| (parser)(i)),
        }
    }

    /// Run the parser contained in this object
    pub fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, Line, VerboseError<&'a str>> {
        (self.parser)(input, self.op)
    }
}

pub fn opcode_name(input: u32) -> Option<&'static str> {
    // early return if the instruction is a nop
    if input == 0 {
        return Some("nop");
    }

    // copied from src/stages/decode.rs
    let op_mask = 0b11111100000000000000000000000000;
    let fn_mask = 0b00000000000000000000000000111111;

    let funct = input & fn_mask;
    let op = (input & op_mask) >> 26;

    let opcode = if op == 0 {
        Opcode::Funct(funct as u8)
    } else {
        Opcode::Op(op as u8)
    };

    match opcode {
        Opcode::Funct(funct) => match funct {
            0x00 => Some("sll"),
            0x02 => Some("srl"),
            0x03 => Some("sra"),
            0x06 => Some("srlv"),
            0x08 => Some("jr"),
            0x0c => Some("syscall"),
            0x1a => Some("div"),
            0x1b => Some("divu"),
            0x20 => Some("add"),
            0x21 => Some("addu"),
            0x22 => Some("sub"),
            0x24 => Some("and"),
            0x25 => Some("or"),
            0x26 => Some("xor"),
            0x27 => Some("nor"),
            0x2a => Some("slt"),
            _ => None,
        },
        Opcode::Op(op) => match op {
            0x02 => Some("j"),
            0x03 => Some("jal"),
            0x04 => Some("beq"),
            0x05 => Some("bne"),
            0x08 => Some("addi"),
            0x09 => Some("addiu"),
            0x0c => Some("andi"),
            0x0d => Some("ori"),
            0x0f => Some("lui"),
            0x23 => Some("lw"),
            0x2b => Some("sw"),
            _ => None,
        },
    }
}

pub fn opcode(input: &str) -> IResult<&str, InstructionParser, VerboseError<&str>> {
    context(
        "Unknown Opcode",
        map_res(
            take_till(|c: char| c.is_whitespace()),
            |word: &str| match word.to_lowercase().trim() {
                "add" => Ok(InstructionParser::new(Opcode::Funct(0x20), r_type)),
                "sub" => Ok(InstructionParser::new(Opcode::Funct(0x22), r_type)),
                "addi" => Ok(InstructionParser::new(Opcode::Op(0x08), i_type)),
                "addiu" => Ok(InstructionParser::new(Opcode::Op(0x09), i_type)),
                "addu" => Ok(InstructionParser::new(Opcode::Funct(0x21), r_type)),
                "and" => Ok(InstructionParser::new(Opcode::Funct(0x24), r_type)),
                "andi" => Ok(InstructionParser::new(Opcode::Op(0x0c), i_type)),
                "beq" => Ok(InstructionParser::new(Opcode::Op(0x04), branch_type)),
                "bne" => Ok(InstructionParser::new(Opcode::Op(0x05), branch_type)),
                "blt" => Ok(InstructionParser::pseudo(|i| multi_branch(i, true, false))),
                "bgt" => Ok(InstructionParser::pseudo(|i| multi_branch(i, false, false))),
                "ble" => Ok(InstructionParser::pseudo(|i| multi_branch(i, true, true))),
                "bge" => Ok(InstructionParser::pseudo(|i| multi_branch(i, false, true))),
                "div" => Ok(InstructionParser::new(Opcode::Funct(0x1a), NO_PARSER)),
                "divu" => Ok(InstructionParser::new(Opcode::Funct(0x1b), NO_PARSER)),
                "j" => Ok(InstructionParser::new(Opcode::Op(0x02), j_type)),
                "jal" => Ok(InstructionParser::new(Opcode::Op(0x03), j_type)),
                "jr" => Ok(InstructionParser::new(Opcode::Funct(0x08), jr_type)),
                "lw" => Ok(InstructionParser::new(Opcode::Op(0x23), load_type)),
                "sw" => Ok(InstructionParser::new(Opcode::Op(0x2b), load_type)),
                "lui" => Ok(InstructionParser::new(Opcode::Op(0x0f), lui)),
                "slt" => Ok(InstructionParser::new(Opcode::Funct(0x2a), NO_PARSER)),
                "ori" => Ok(InstructionParser::new(Opcode::Op(0x0d), i_type)),
                "or" => Ok(InstructionParser::new(Opcode::Funct(0x25), r_type)),
                "xor" => Ok(InstructionParser::new(Opcode::Funct(0x26), r_type)),
                "nor" => Ok(InstructionParser::new(Opcode::Funct(0x27), r_type)),
                "sll" => Ok(InstructionParser::new(Opcode::Funct(0x0), shift_type)),
                "srl" => Ok(InstructionParser::new(Opcode::Funct(0x2), shift_type)),
                "sra" => Ok(InstructionParser::new(Opcode::Funct(0x3), shift_type)),
                "srlv" => Ok(InstructionParser::new(Opcode::Funct(0x6), r_type)),
                "move" => Ok(InstructionParser::pseudo(move_ins)),
                "li" => Ok(InstructionParser::pseudo(li_ins)),
                "la" => Ok(InstructionParser::pseudo(li_ins)),
                "syscall" => Ok(InstructionParser::pseudo(syscall)),
                "nop" => Ok(InstructionParser::pseudo(nop)),
                ".word" => Ok(InstructionParser::pseudo(word_lit)),
                ".ascii" => Ok(InstructionParser::pseudo(ascii_lit)),
                ".asciiz" => Ok(InstructionParser::pseudo(asciiz_lit)),
                ".text" => Ok(InstructionParser::pseudo(|i| segment(i, Segment::Text))),
                ".data" => Ok(InstructionParser::pseudo(|i| segment(i, Segment::Data))),
                _ => Err(()),
            },
        ),
    )(input)
}
