use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alphanumeric1, multispace0},
    combinator::{map, map_res, opt},
    error::VerboseError,
    multi::many1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{
    parser::{self, model::Opcode},
    Register, AT, ZERO,
};

use super::{
    model::{Imm, Instruction, Line, Symbol},
    opcode::Style,
    ParseError,
};

fn immediate(input: &str) -> IResult<&str, Imm, VerboseError<&str>> {
    preceded(
        multispace0,
        alt((
            map(parser::int, |x: i64| Imm::Value(x)),
            map(alphanumeric1, |x: &str| Imm::Label(x.to_string())),
        )),
    )(input)
}

fn symbol(input: &str) -> IResult<&str, Symbol, VerboseError<&str>> {
    preceded(
        multispace0,
        alt((
            map(parser::int, |x: u32| Symbol::Address(x)),
            map(alphanumeric1, |x: &str| Symbol::Label(x.to_string())),
        )),
    )(input)
}

fn j_type(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    // j format instructions should look like
    // <OP> <ADDRESS>
    let (input, (op, style)) = parser::opcode(input)?;
    let (input, addr) = symbol(input)?;
    Ok((input, vec![Instruction::J { op, addr }]))
}

fn r_type(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    // all r format instruction should look like
    // op rd, rs, rt
    let (input, (op, style)) = parser::opcode(input)?;
    let (input, (rd, rs, rt)) =
        tuple((parser::register, parser::register, parser::register))(input)?;
    // TODO: support shift instructions
    Ok((
        input,
        vec![Instruction::R {
            op,
            rd,
            rs,
            rt,
            shamt: 0,
        }],
    ))
}

fn i_type(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    let (input, (op, style)) = parser::opcode(input)?;
    let (input, _) = opt(tag(","))(input)?;
    let (input, rt) = parser::register(input)?;
    let (input, _) = multispace0(input)?;

    // options for second arg
    // add style: rs, imm
    // mem style: imm(rs)
    // TODO: I'm not going to do style checking for now
    // technically something like `lw $t0, $t2, 4` or `addi $t0, 1($t1)`
    // will be accepted

    let add_style = tuple((parser::register, preceded(multispace0, immediate)))(input);

    let rs;
    let imm;
    let fin_input;

    if let Ok(add_style) = add_style {
        // extract data
        let (input, (reg, mut num)): (&str, (Register, Imm)) = add_style;
        if let Style::PcRelative = style {
            if let Imm::Label(label) = num {
                num = Imm::PcRelative(label);
            }
        }
        rs = reg;
        imm = num;
        fin_input = input;
    } else {
        // try other style
        let (input, (num, reg)): (&str, (i64, Register)) =
            tuple((parser::int, delimited(tag("("), parser::register, tag(")"))))(input)?;
        rs = reg;
        imm = Imm::Value(num);
        fin_input = input;
    }

    Ok((fin_input, vec![Instruction::I { op, rt, rs, imm }]))
}

/// Currently supported pseudoinstructions:
/// - `li` : Load immediate
/// - `la` : Load address
/// - `move` : Move address
fn move_ins(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    map(
        preceded(tag("move"), tuple((parser::register, parser::register))),
        |regs| {
            vec![Instruction::R {
                op: Opcode::Funct(0x20), // add
                rd: regs.0,
                rs: regs.1,
                rt: ZERO,
                shamt: 0,
            }]
        },
    )(input)
}

fn li_ins(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    // li and la are the same
    map(
        preceded(
            alt((tag("li"), tag("la"))),
            tuple((parser::register, immediate)),
        ),
        |(reg, imm)| match imm {
            Imm::Label(ref name) => vec![
                // TODO: Make loads >16bits work
                // Instruction::I {
                //     op: Opcode::Op(0x0f),
                //     rt: AT,
                //     rs: ZERO,
                //     imm: Imm::HighHWord(name.clone()),
                // },
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
    )(input)
}

pub fn syscall(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    let (input, _) = delimited(multispace0, tag("syscall"), multispace0)(input)?;
    Ok((
        input,
        vec![Instruction::R {
            op: Opcode::Funct(0x0c),
            rd: ZERO,
            rs: ZERO,
            rt: ZERO,
            shamt: 0,
        }],
    ))
}

pub fn word_lit(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    preceded(
        tag(".word"),
        many1(map(
            delimited(multispace0, parser::int, opt(tag(","))),
            |i: i64| Instruction::Literal { data: i as u32 },
        )),
    )(input)
}

pub fn nop(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    let (input, _) = delimited(multispace0, tag("nop"), multispace0)(input)?;
    Ok((input, vec![Instruction::Literal { data: 0 }]))
}

pub fn ascii_lit(input: &str) -> IResult<&str, Vec<Instruction>, VerboseError<&str>> {
    preceded(
        tag(".ascii"),
        delimited(
            multispace0,
            map(
                delimited(tag("\""), take_till(|c: char| c == '"'), tag("\"")),
                |s: &str| {
                    // escape stuff
                    let s = s.replace("\\n", "\n");
                    let s = s.replace("\\0", "\0");
                    // convert to bytes
                    let bytes = s.as_bytes();
                    let mut words = vec![];
                    while words.len() * 4 < bytes.len() {
                        let i = words.len() * 4;
                        words.push(Instruction::Literal {
                            data: u32::from_ne_bytes([
                                *bytes.get(i).unwrap_or(&0),
                                *bytes.get(i + 1).unwrap_or(&0),
                                *bytes.get(i + 2).unwrap_or(&0),
                                *bytes.get(i + 3).unwrap_or(&0),
                            ]),
                        });
                    }
                    words
                },
            ),
            opt(tag(",")),
        ),
    )(input)
}

pub fn instruction(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    map(
        alt((
            r_type, i_type, j_type, nop, move_ins, li_ins, syscall, word_lit, ascii_lit,
        )),
        |x| Line::Instruction(x),
    )(input)
}
