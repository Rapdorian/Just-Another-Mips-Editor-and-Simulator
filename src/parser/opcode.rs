use super::model::Opcode;
use super::ParseError;
use nom::{bytes::complete::take_till, combinator::map_res, IResult};

pub fn opcode(input: &str) -> IResult<&str, Opcode> {
    map_res(
        take_till(|c: char| c.is_whitespace()),
        |name: &str| match name.to_lowercase().trim() {
            "add" => Ok(Opcode::Funct(0x20)),
            "sub" => Ok(Opcode::Funct(0x22)),
            "addi" => Ok(Opcode::Op(0x08)),
            "addiu" => Ok(Opcode::Op(0x09)),
            "addu" => Ok(Opcode::Funct(0x21)),
            "and" => Ok(Opcode::Funct(0x24)),
            "andi" => Ok(Opcode::Op(0x0c)),
            "beq" => Ok(Opcode::Op(0x04)),
            "bne" => Ok(Opcode::Op(0x05)),
            "blez" => Ok(Opcode::Op(0x06)),
            "bgtz" => Ok(Opcode::Op(0x07)),
            "div" => Ok(Opcode::Funct(0x1a)),
            "divu" => Ok(Opcode::Funct(0x1b)),
            "j" => Ok(Opcode::Op(0x02)),
            "jal" => Ok(Opcode::Op(0x03)),
            "jalr" => Ok(Opcode::Funct(0x09)),
            "jr" => Ok(Opcode::Funct(0x08)),
            "lw" => Ok(Opcode::Op(0x23)),
            "sw" => Ok(Opcode::Op(0x2b)),
            "lui" => Ok(Opcode::Op(0x0f)),
            "slt" => Ok(Opcode::Funct(0x2a)),
            _ => Err(ParseError::UnknownInstruction(name.to_string())),
        },
    )(input)
}
