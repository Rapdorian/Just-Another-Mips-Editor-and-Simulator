//! TODO: Needs to be able to handle pseudo-instructions and comments

use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::multispace0,
    multi::many0,
    sequence::{delimited, preceded},
    Finish, IResult,
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

pub fn parse_line(input: &str) -> IResult<&str, Line> {
    delimited(multispace0, alt((instruction, label)), multispace0)(input)
}

pub fn parse_string(input: &str) -> Result<Vec<Line>, nom::error::Error<String>> {
    match many0(parse_line)(input).finish() {
        Ok((rem, output)) => {
            println!("Unparsed remainder: {}", rem);
            Ok(output)
        }
        Err(e) => Err(nom::error::Error {
            //Ugly hack to I don't have to borrow input statically
            input: e.input.to_string(),
            code: e.code,
        }),
    }
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
