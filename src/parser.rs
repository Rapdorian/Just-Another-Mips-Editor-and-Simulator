//! TODO: Needs to be able to handle pseudo-instructions and comments

use std::{collections::HashMap, ops::Deref};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::complete::{multispace0, space0, space1},
    combinator::{eof, map},
    error::{context, VerboseError, VerboseErrorKind},
    multi::many_till,
    sequence::{delimited, preceded, terminated},
    Finish, IResult,
};
use thiserror::Error;

mod directives;
mod instruction;
mod label;
pub mod model;
mod numbers;
mod opcode;
mod register;

pub use instruction::instruction;
pub use label::label;
pub use numbers::*;
pub use opcode::opcode;
pub use register::register;

use model::{LabelTable, Line};

use self::model::{Segment, Segments};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown instruction: '{0}'")]
    UnknownInstruction(String),
}

/// Converts an error trace into a usable error message
fn convert_error<I>(input: I, error: VerboseError<I>) -> String
where
    I: Deref<Target = str>,
{
    // remove nom-specific messages
    let mut errors = vec![];

    for error in error.errors {
        match error.1 {
            VerboseErrorKind::Context(_) => {
                errors.push(error);
            }
            VerboseErrorKind::Char(_) => {
                errors.push(error);
            }
            VerboseErrorKind::Nom(_) => {
                // only use nom errors in debug mode
                #[cfg(debug_assertions)]
                errors.push(error);
            }
        }
    }
    nom::error::convert_error(input, VerboseError { errors })
}

pub fn comment(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    context(
        "Parsing comment",
        map(
            preceded(
                preceded(multispace0, context("Comments begin with a #", tag("#"))),
                context("Comment body", take_while(|c| c != '\n')),
            ),
            |x: &str| Line::Comment(x.to_string()),
        ),
    )(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    context(
        "Parsing Line",
        delimited(
            multispace0,
            instruction,
            context(
                "Instructions must be on their own lines",
                preceded(space0, alt((tag("\n"), eof, map(comment, |_| "")))),
            ),
        ),
    )(input)
}

pub fn parse_string(input: &str) -> Result<Vec<Line>> {
    let (_, (output, _)) = many_till(alt((comment, label, parse_line)), eof)(input)
        .finish()
        .map_err(|e| anyhow!("{}", convert_error(input, e)))?;
    Ok(output)
}

pub fn compute_labels(input: &[Line]) -> LabelTable {
    let mut labels = HashMap::new();
    let mut segments = Segments::default();
    let mut pc = segments.switch(Segment::Text);

    for line in input {
        match line {
            Line::Label(name) => {
                labels.insert(name.clone(), *pc);
            }
            Line::Instruction(ins) => {
                *pc += ins.len() as u32 * 4;
            }
            Line::Segment(seg) => pc = segments.switch(*seg),
            Line::Comment(_) => {}
        }
    }
    labels
}
