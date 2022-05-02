//! TODO: Needs to be able to handle pseudo-instructions and comments

use std::ops::Deref;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::space0,
    combinator::{eof, map, opt},
    error::{context, VerboseError, VerboseErrorKind},
    multi::many_till,
    sequence::{delimited, preceded, terminated},
    Finish, IResult,
};

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
pub use opcode::opcode_name;
pub use register::register;

use model::{LabelTable, Line};

use self::model::{Segment, Segments};

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

pub fn blank(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    let (input, _) = preceded(space0, alt((tag("\n"), eof, map(comment, |_| ""))))(input)?;
    Ok((input, Line::Blank))
}

pub fn comment(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    context(
        "Parsing comment",
        map(
            delimited(
                preceded(space0, context("Comments begin with a #", tag("#"))),
                context("Comment body", take_while(|c| c != '\n')),
                tag("\n"),
            ),
            |x: &str| Line::Comment(x.to_string()),
        ),
    )(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    context(
        "Parsing Line",
        delimited(
            space0,
            instruction,
            context(
                "Instructions must be on their own lines",
                preceded(space0, alt((tag("\n"), eof, map(comment, |_| "")))),
            ),
        ),
    )(input)
}

pub fn parse_string(input: &str) -> Result<Vec<Line>> {
    let (_, (output, _)) = many_till(
        alt((
            comment,
            blank,
            terminated(label, preceded(space0, opt(tag("\n")))),
            parse_line,
        )),
        eof,
    )(input)
    .finish()
    .map_err(|e| anyhow!("{}", convert_error(input, e)))?;
    Ok(output)
}

pub fn compute_labels(input: &[Line]) -> LabelTable {
    let mut labels = LabelTable::default();
    let mut segments = Segments::default();
    let mut pc = segments.switch(Segment::Text);

    for (i, line) in input.iter().enumerate() {
        match line {
            Line::Label(name) => {
                labels.insert_label(name.clone(), *pc);
            }
            Line::Instruction(ins) => {
                labels.insert_line(i, *pc);
                let mut len = 0;
                for inst in ins {
                    len += match inst {
                        model::Instruction::Literal { data } => data.len(),
                        _ => 4,
                    };
                }
                *pc += len as u32;
            }
            Line::Segment(seg) => pc = segments.switch(*seg),
            _ => {}
        }
    }

    std::fs::write("labels.txt", format!("{:#?}", input)).unwrap();
    labels
}
