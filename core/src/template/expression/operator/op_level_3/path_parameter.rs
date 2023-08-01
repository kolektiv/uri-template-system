use nom::{
    character::complete as character,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

#[derive(Clone, Debug, PartialEq)]
pub struct PathParameter;

// -----------------------------------------------------------------------------

// Parsing

impl PathParameter {
    pub fn parse(input: &str) -> IResult<&str, PathParameter> {
        character::char(';').value(PathParameter).parse(input)
    }
}
