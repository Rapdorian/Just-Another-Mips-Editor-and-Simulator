use super::directives::{ascii_lit, segment, word_lit};
use super::instruction::{
    branch_type, i_type, j_type, jr_type, li_ins, load_type, lui, move_ins, nop, r_type, syscall,
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
                "blez" => Ok(InstructionParser::new(Opcode::Op(0x06), branch_type)),
                "bgtz" => Ok(InstructionParser::new(Opcode::Op(0x07), branch_type)),
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
                "move" => Ok(InstructionParser::pseudo(move_ins)),
                "li" => Ok(InstructionParser::pseudo(li_ins)),
                "la" => Ok(InstructionParser::pseudo(li_ins)),
                "syscall" => Ok(InstructionParser::pseudo(syscall)),
                "nop" => Ok(InstructionParser::pseudo(nop)),
                ".word" => Ok(InstructionParser::pseudo(word_lit)),
                ".ascii" => Ok(InstructionParser::pseudo(ascii_lit)),
                ".text" => Ok(InstructionParser::pseudo(|i| segment(i, Segment::Text))),
                ".data" => Ok(InstructionParser::pseudo(|i| segment(i, Segment::Data))),
                _ => Err(()),
            },
        ),
    )(input)
}
