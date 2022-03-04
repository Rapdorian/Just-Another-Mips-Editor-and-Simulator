use std::convert::TryFrom;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::multispace0,
    combinator::{map_res, opt},
    error::{context, VerboseError},
    sequence::preceded,
    IResult,
};

use crate::Register;

pub fn register_name(input: &str) -> IResult<&str, Register, VerboseError<&str>> {
    context(
        "Unknown register",
        map_res(
            take_till(|c: char| c.is_whitespace() || c == ',' || c == ')' || c == '#'),
            |name: &str| Register::try_from(name),
        ),
    )(input)
}

pub fn register(input: &str) -> IResult<&str, Register, VerboseError<&str>> {
    let (input, _) = context("Expected '$' to prepend register", tag("$"))(input)?;
    let (input, reg) = register_name(input)?;
    Ok((input, reg))
}
