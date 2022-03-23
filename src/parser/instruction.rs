use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, peek},
    error::{context, VerboseError},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::{
    parser::{self, model::Opcode},
    AT, ZERO,
};

use super::{
    int,
    label::identifier,
    model::{Imm, Instruction, Line, Symbol},
};

fn immediate(input: &str) -> IResult<&str, Imm, VerboseError<&str>> {
    preceded(
        space0,
        alt((
            map(parser::int, |x: i64| Imm::Value(x)),
            map(identifier, |x: &str| Imm::Label(x.to_string())),
        )),
    )(input)
}

fn separator(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("separator", delimited(space0, tag(","), space0))(input)
}

fn symbol(input: &str) -> IResult<&str, Symbol, VerboseError<&str>> {
    preceded(
        space0,
        alt((
            map(parser::int, |x: u32| Symbol::Address(x)),
            map(identifier, |x: &str| Symbol::Label(x.to_string())),
        )),
    )(input)
}

pub(crate) type ParserOutput<'a> = IResult<&'a str, Line, VerboseError<&'a str>>;

/// Parse jump instructions
/// <OP> <label>
pub fn j_type(input: &str, op: Opcode) -> ParserOutput {
    let (input, addr) = context("Expected label", symbol)(input)?;
    Ok((
        input,
        Line::Instruction(vec![
            Instruction::J { op, addr },
            Instruction::Literal { data: 0 },
            Instruction::Literal { data: 0 },
        ]),
    ))
}

/// Parse JR instruction
pub fn jr_type(input: &str, op: Opcode) -> ParserOutput {
    let (input, rs) = context("Expected register", parser::register)(input)?;
    let rt = ZERO;
    let rd = ZERO;
    Ok((
        input,
        Line::Instruction(vec![
            Instruction::R {
                op,
                rd,
                rs,
                rt,
                shamt: 0,
            },
            Instruction::Literal { data: 0 },
            Instruction::Literal { data: 0 },
        ]),
    ))
}

/// Parses simple R-type instructions using the format
/// `<OP> <rd>, <rs>, <rt>`
pub fn r_type(input: &str, op: Opcode) -> ParserOutput {
    let (input, rd) = context("Destination Register", parser::register)(input)?;
    let (input, rs) = context("Source Register", preceded(separator, parser::register))(input)?;
    let (input, rt) = context("Target Register", preceded(separator, parser::register))(input)?;
    Ok((
        input,
        Line::Instruction(vec![Instruction::R {
            op,
            rd,
            rs,
            rt,
            shamt: 0,
        }]),
    ))
}

/// Parses shift style instructions
/// `<OP> <rd>, <rs>, shamt`
pub fn shift_type(input: &str, op: Opcode) -> ParserOutput {
    let (input, rd) = context("Expected Destination register", parser::register)(input)?;
    let (input, rt) = context(
        "Expected Target register",
        preceded(separator, parser::register),
    )(input)?;
    let (input, shamt): (&str, i64) =
        context("Expected shift amount", preceded(separator, int))(input)?;
    let shamt = shamt as u32;
    Ok((
        input,
        Line::Instruction(vec![Instruction::R {
            op,
            rd,
            rs: ZERO,
            rt,
            shamt,
        }]),
    ))
}

/// Parses simple immediate mode instructions using the format
/// `<OP> <rt> <rs> <imm>`
pub fn i_type(input: &str, op: Opcode) -> ParserOutput {
    let (input, rt) = context("Expected target register", parser::register)(input)?;
    let (input, rs) = context(
        "Expected source register",
        preceded(separator, parser::register),
    )(input)?;
    let (input, imm) = context("Expected immediate value", preceded(separator, immediate))(input)?;
    Ok((
        input,
        Line::Instruction(vec![Instruction::I { op, rt, rs, imm }]),
    ))
}

/// Parses lui instruction
/// `<OP> <rt> <imm>`
pub fn lui(input: &str, op: Opcode) -> ParserOutput {
    let (input, rt) = context("Expected target register", parser::register)(input)?;
    let (input, imm) = context("Expected immediate value", preceded(separator, immediate))(input)?;
    Ok((
        input,
        Line::Instruction(vec![Instruction::I {
            op,
            rt,
            rs: ZERO,
            imm,
        }]),
    ))
}

/// Parses load and store instructions
/// `<OP> <rt> <imm>(<rs>)
pub fn load_type(input: &str, op: Opcode) -> ParserOutput {
    let (input, rt) = context("Expected target register", parser::register)(input)?;
    let (input, imm) = context("Expected offset value", preceded(separator, immediate))(input)?;
    let (input, rs) = context(
        "Expected source value",
        delimited(tag("("), parser::register, tag(")")),
    )(input)?;
    Ok((
        input,
        Line::Instruction(vec![Instruction::I { op, rt, rs, imm }]),
    ))
}

/// Parses branch instructions
/// `<OP> <rt> <rs> <label>`
pub fn branch_type(input: &str, op: Opcode) -> ParserOutput {
    let (input, rt) = context("Expected first register", parser::register)(input)?;
    let (input, rs) = context(
        "Expected second register",
        preceded(separator, parser::register),
    )(input)?;
    let (input, mut imm) = context("Expected label", preceded(separator, immediate))(input)?;

    // if we got a label make it pc relative
    if let Imm::Label(label) = imm {
        imm = Imm::PcRelative(label);
    }
    Ok((
        input,
        Line::Instruction(vec![
            Instruction::I { op, rt, rs, imm },
            Instruction::Literal { data: 0 },
            Instruction::Literal { data: 0 },
        ]),
    ))
}

/// Parses a move pseudoinstruction
pub fn move_ins(input: &str) -> ParserOutput {
    let (input, rd) = context("Expected destination register", parser::register)(input)?;
    let (input, rs) = context(
        "Expected source register",
        preceded(separator, parser::register),
    )(input)?;
    Ok((
        input,
        Line::Instruction(vec![Instruction::R {
            op: Opcode::Funct(0x20), // add
            rd,
            rs,
            rt: ZERO,
            shamt: 0,
        }]),
    ))
}

/// Parses li and la instructions
pub fn li_ins(input: &str) -> ParserOutput {
    // li and la are the same
    map(
        map(
            tuple((parser::register, preceded(separator, immediate))),
            |(reg, imm)| match imm {
                Imm::Label(ref name) => vec![
                    // TODO: Make loads >16bits work
                    Instruction::I {
                        op: Opcode::Op(0x0f),
                        rt: AT,
                        rs: ZERO,
                        imm: Imm::HighHWord(name.clone()),
                    },
                    Instruction::I {
                        op: Opcode::Op(0x0d),
                        rt: reg,
                        rs: AT,
                        imm: Imm::LowHWord(name.clone()),
                    },
                ],
                Imm::HighHWord(_) => vec![Instruction::I {
                    op: Opcode::Op(0x0f),
                    rt: reg,
                    rs: ZERO,
                    imm,
                }],
                Imm::LowHWord(_) => vec![Instruction::I {
                    op: Opcode::Op(0x08), //addi
                    rt: reg,
                    rs: ZERO,
                    imm,
                }],
                Imm::Value(value) => {
                    if value > u16::MAX as i64 {
                        vec![
                            Instruction::I {
                                op: Opcode::Op(0x0f),
                                rt: AT,
                                rs: ZERO,
                                imm: Imm::Value((value & 0xFFFF0000) >> 16),
                            },
                            Instruction::I {
                                op: Opcode::Op(0x0d),
                                rt: reg,
                                rs: AT,
                                imm: Imm::Value(value & 0xFFFF),
                            },
                        ]
                    } else {
                        vec![Instruction::I {
                            op: Opcode::Op(0x08), // addi
                            rt: reg,
                            rs: ZERO,
                            imm,
                        }]
                    }
                }
                Imm::PcRelative(_) => todo!(),
            },
        ),
        |x| Line::Instruction(x),
    )(input)
}

pub fn syscall(input: &str) -> ParserOutput {
    Ok((
        input,
        Line::Instruction(vec![Instruction::R {
            op: Opcode::Funct(0x0c),
            rd: ZERO,
            rs: ZERO,
            rt: ZERO,
            shamt: 0,
        }]),
    ))
}

pub fn nop(input: &str) -> ParserOutput {
    Ok((
        input,
        Line::Instruction(vec![Instruction::Literal { data: 0 }]),
    ))
}

pub fn instruction(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    // grab the opcode
    let (input, parser) = preceded(
        space0,
        terminated(parser::opcode, alt((space1, peek(tag("\n"))))),
    )(input)?;
    parser.parse(input)
}
