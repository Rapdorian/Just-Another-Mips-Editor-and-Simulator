use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{map, recognize},
    error::{context, VerboseError},
    multi::many0,
    sequence::{pair, preceded, terminated},
    IResult,
};

use super::model::Line;

/// Parses a mips identifier currently uses the following format
/// [_A-z][_A-z0-9]*
pub fn identifier(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn label(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    context(
        "Label",
        map(
            preceded(
                multispace0,
                terminated(
                    context("Identifier", identifier),
                    context("Label must be terminated by a colon", tag(":")),
                ),
            ),
            |label: &str| Line::Label(label.to_string()),
        ),
    )(input)
}
