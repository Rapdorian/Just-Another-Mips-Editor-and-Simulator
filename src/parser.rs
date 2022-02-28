//! TODO: Needs to be able to handle pseudo-instructions and comments

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt, character::complete::multispace0, combinator::eof, error::VerboseError,
    multi::many_till, sequence::delimited, Finish, IResult,
};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown instruction: '{0}'")]
    UnknownInstruction(String),
}

pub fn parse_line(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    delimited(multispace0, alt((instruction, label)), multispace0)(input)
}

pub fn parse_string(input: &str) -> Result<Vec<Line>> {
    let (_, (output, _)) = many_till(parse_line, eof)(input)
        .finish()
        .map_err(|e| anyhow!("{}", e))?;
    Ok(output)
}

pub fn compute_labels(input: &[Line]) -> LabelTable {
    let mut labels = HashMap::new();
    let mut pc = 0;

    for line in input {
        match line {
            Line::Label(name) => {
                labels.insert(name.clone(), pc);
            }
            Line::Instruction(ins) => {
                pc += ins.len() * 4;
            }
        }
    }
    labels
}
