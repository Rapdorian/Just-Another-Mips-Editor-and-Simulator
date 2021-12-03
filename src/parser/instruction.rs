use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alphanumeric1, multispace0},
    combinator::{map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{
    parser::{self, model::Opcode},
    Register, AT, ZERO,
};

use super::{
    model::{Imm, Instruction, Line},
    ParseError,
};

fn immediate(input: &str) -> IResult<&str, Imm> {
    preceded(
        multispace0,
        alt((
            map(parser::int, |x: i64| Imm::Value(x as u32)),
            map(alphanumeric1, |x: &str| Imm::Label(x.to_string())),
        )),
    )(input)
}

fn r_type(input: &str) -> IResult<&str, Vec<Instruction>> {
    // all r format instruction should look like
    // op rd, rs, rt
    let (input, op) = parser::opcode(input)?;
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

fn i_type(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, op) = parser::opcode(input)?;
    let (input, _) = opt(tag(","))(input)?;
    let (input, rt) = parser::register(input)?;
    let (input, _) = multispace0(input)?;

    // options for second arg
    // add style: rs, imm
    // mem style: imm(rs)
    // TODO: I'm not going to do style checking for now
    // technically something like `lw $t0, $t2, 4` or `addi $t0, 1($t1)`
    // will be accepted

    let add_style = tuple((parser::register, preceded(multispace0, parser::int)))(input);

    let rs;
    let imm;
    let fin_input;

    if let Ok(add_style) = add_style {
        // extract data
        let (input, (reg, num)): (&str, (Register, i16)) = add_style;
        rs = reg;
        imm = num;
        fin_input = input;
    } else {
        // try other style
        let (input, (num, reg)): (&str, (i16, Register)) =
            tuple((parser::int, delimited(tag("("), parser::register, tag(")"))))(input)?;
        rs = reg;
        imm = num;
        fin_input = input;
    }

    Ok((
        fin_input,
        vec![Instruction::I {
            op,
            rt,
            rs,
            imm: Imm::Value(imm as u32),
        }],
    ))
}

/// Currently supported pseudoinstructions:
/// - `li` : Load immediate
/// - `la` : Load address
/// - `move` : Move address
fn move_ins(input: &str) -> IResult<&str, Vec<Instruction>> {
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

fn li_ins(input: &str) -> IResult<&str, Vec<Instruction>> {
    // li and la are the same
    map(
        preceded(
            alt((tag("li"), tag("la"))),
            tuple((parser::register, immediate)),
        ),
        |(reg, imm)| match imm {
            Imm::Label(ref name) => vec![
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
                if value > u16::MAX as u32 {
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
        },
    )(input)
}

pub fn syscall(input: &str) -> IResult<&str, Vec<Instruction>> {
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

pub fn instruction(input: &str) -> IResult<&str, Line> {
    map(alt((r_type, i_type, move_ins, li_ins, syscall)), |x| {
        Line::Instruction(x)
    })(input)
}
