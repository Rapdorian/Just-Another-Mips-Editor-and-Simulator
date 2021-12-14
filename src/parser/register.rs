use std::convert::TryFrom;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::multispace0,
    combinator::{map_res, opt},
    sequence::preceded,
    IResult,
};

use crate::Register;

pub fn register_name(input: &str) -> IResult<&str, Register> {
    map_res(
        take_till(|c: char| c.is_whitespace() || c == ',' || c == ')' || c == '#'),
        |name: &str| Register::try_from(name),
    )(input)
}

pub fn register(input: &str) -> IResult<&str, Register> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("$")(input)?;
    let (input, reg) = register_name(input)?;
    let (input, _) = opt(tag(","))(input)?;
    Ok((input, reg))
}
