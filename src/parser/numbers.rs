use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    error::VerboseError,
    multi::{many0, many1},
    sequence::{preceded, terminated},
    IResult,
};
use num::Num;

pub fn num_str(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    recognize(many1(terminated(
        one_of("-+0123456789ABCDEF"),
        many0(char('_')),
    )))(input)
}

pub fn dec<T: Num>(input: &str) -> IResult<&str, T, VerboseError<&str>> {
    map_res(num_str, |s: &str| T::from_str_radix(s, 10))(input)
}

pub fn hex<T: Num>(input: &str) -> IResult<&str, T, VerboseError<&str>> {
    preceded(
        tag("0x"),
        map_res(num_str, |s: &str| T::from_str_radix(s, 16)),
    )(input)
}

pub fn oct<T: Num>(input: &str) -> IResult<&str, T, VerboseError<&str>> {
    preceded(
        tag("0o"),
        map_res(num_str, |s: &str| T::from_str_radix(s, 8)),
    )(input)
}

pub fn bin<T: Num>(input: &str) -> IResult<&str, T, VerboseError<&str>> {
    preceded(
        tag("0b"),
        map_res(num_str, |s: &str| T::from_str_radix(s, 2)),
    )(input)
}

pub fn int<T: Num>(input: &str) -> IResult<&str, T, VerboseError<&str>> {
    alt((hex, oct, bin, dec))(input)
}
