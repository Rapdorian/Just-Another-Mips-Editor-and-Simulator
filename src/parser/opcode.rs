use super::instruction::{
    ascii_lit, branch_type, i_type, j_type, jr_type, li_ins, load_type, move_ins, nop, r_type,
    syscall, word_lit,
};
use super::model::{Instruction, Opcode};
use super::ParseError;
use nom::combinator::fail;
use nom::error::{context, ErrorKind, FromExternalError, VerboseError};
use nom::{bytes::complete::take_till, combinator::map_res, IResult};

type InsParser = fn(&str, Opcode) -> IResult<&str, Vec<Instruction>, VerboseError<&str>>;

const NO_PARSER: InsParser = |input, _| context("No parser for instruction", fail)(input);

pub struct InstructionParser {
    op: Opcode,
    parser: Box<dyn Fn(&str, Opcode) -> IResult<&str, Vec<Instruction>, VerboseError<&str>>>,
}

impl InstructionParser {
    pub fn new<F>(op: Opcode, parser: F) -> Self
    where
        F: Fn(&str, Opcode) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> + 'static,
    {
        Self {
            op,
            parser: Box::new(parser),
        }
    }

    pub fn pseudo<F>(parser: F) -> Self
    where
        F: Fn(&str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> + 'static,
    {
        Self {
            op: Opcode::Op(0),
            parser: Box::new(move |i, _| (parser)(i)),
        }
    }

    pub fn parse<'a>(
        &self,
        input: &'a str,
    ) -> IResult<&'a str, Vec<Instruction>, VerboseError<&'a str>> {
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
                "lui" => Ok(InstructionParser::new(Opcode::Op(0x0f), NO_PARSER)),
                "slt" => Ok(InstructionParser::new(Opcode::Funct(0x2a), NO_PARSER)),
                "move" => Ok(InstructionParser::pseudo(move_ins)),
                "li" => Ok(InstructionParser::pseudo(li_ins)),
                "la" => Ok(InstructionParser::pseudo(li_ins)),
                "syscall" => Ok(InstructionParser::pseudo(syscall)),
                "nop" => Ok(InstructionParser::pseudo(nop)),
                ".word" => Ok(InstructionParser::pseudo(word_lit)),
                ".ascii" => Ok(InstructionParser::pseudo(ascii_lit)),
                _ => Err(()),
            },
        ),
    )(input)
}
