use nom::{
    bytes::complete::{tag, take_till},
    character::complete::multispace0,
    combinator::{map, opt},
    multi::many1,
    sequence::delimited,
};

use crate::parser;

use super::{
    instruction::ParserOutput,
    model::{Instruction, Line, Segment},
};

pub fn ascii_lit(input: &str) -> ParserOutput {
    map(
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
        |x| Line::Instruction(x),
    )(input)
}

pub fn asciiz_lit(input: &str) -> ParserOutput {
    map(
        delimited(
            multispace0,
            map(
                delimited(tag("\""), take_till(|c: char| c == '"'), tag("\"")),
                |s: &str| {
                    // escape stuff
                    let s = s.replace("\\n", "\n");
                    let s = s.replace("\\0", "\0");
                    let s = &format!("{s}\0");
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
        |x| Line::Instruction(x),
    )(input)
}

pub fn word_lit(input: &str) -> ParserOutput {
    map(
        many1(map(
            delimited(multispace0, parser::int, opt(tag(","))),
            |i: i64| Instruction::Literal { data: i as u32 },
        )),
        |x| Line::Instruction(x),
    )(input)
}

pub fn segment(input: &str, seg: Segment) -> ParserOutput {
    Ok((input, Line::Segment(seg)))
}
