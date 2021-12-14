use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, combinator::map,
    sequence::terminated, IResult,
};

use super::model::Line;

pub fn label(input: &str) -> IResult<&str, Line> {
    map(terminated(alphanumeric1, tag(":")), |label: &str| {
        Line::Label(label.to_string())
    })(input)
}
