use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, combinator::map, error::VerboseError,
    sequence::terminated, IResult,
};

use super::model::Line;

pub fn label(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    map(terminated(alphanumeric1, tag(":")), |label: &str| {
        Line::Label(label.to_string())
    })(input)
}
