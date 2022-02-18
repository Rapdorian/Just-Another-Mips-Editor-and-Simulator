use super::model::Opcode;
use super::ParseError;
use nom::{
    bytes::complete::take_till,
    combinator::map_res,
    error::{context, VerboseError},
    IResult,
};

pub enum Style {
    Literal,
    Offset,
    PcRelative,
    None,
}

pub fn opcode(input: &str) -> IResult<&str, (Opcode, Style), VerboseError<&str>> {
    context(
        "Unrecognized opcode",
        map_res(
            take_till(|c: char| c.is_whitespace()),
            |name: &str| match name.to_lowercase().trim() {
                "add" => Ok((Opcode::Funct(0x20), Style::None)),
                "sub" => Ok((Opcode::Funct(0x22), Style::None)),
                "addi" => Ok((Opcode::Op(0x08), Style::Literal)),
                "addiu" => Ok((Opcode::Op(0x09), Style::Literal)),
                "addu" => Ok((Opcode::Funct(0x21), Style::None)),
                "and" => Ok((Opcode::Funct(0x24), Style::None)),
                "andi" => Ok((Opcode::Op(0x0c), Style::Literal)),
                "beq" => Ok((Opcode::Op(0x04), Style::PcRelative)),
                "bne" => Ok((Opcode::Op(0x05), Style::PcRelative)),
                "blez" => Ok((Opcode::Op(0x06), Style::PcRelative)),
                "bgtz" => Ok((Opcode::Op(0x07), Style::PcRelative)),
                "div" => Ok((Opcode::Funct(0x1a), Style::None)),
                "divu" => Ok((Opcode::Funct(0x1b), Style::None)),
                "j" => Ok((Opcode::Op(0x02), Style::None)),
                "jal" => Ok((Opcode::Op(0x03), Style::None)),
                "jalr" => Ok((Opcode::Funct(0x09), Style::None)),
                "jr" => Ok((Opcode::Funct(0x08), Style::None)),
                "lw" => Ok((Opcode::Op(0x23), Style::Offset)),
                "sw" => Ok((Opcode::Op(0x2b), Style::Offset)),
                "lui" => Ok((Opcode::Op(0x0f), Style::Offset)),
                "slt" => Ok((Opcode::Funct(0x2a), Style::Literal)),
                _ => Err(ParseError::UnknownInstruction(name.to_string())),
            },
        ),
    )(input)
}
