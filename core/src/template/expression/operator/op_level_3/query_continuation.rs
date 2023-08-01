use nom::{
    character::complete as character,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

#[derive(Clone, Debug, PartialEq)]
pub struct QueryContinuation;

// -----------------------------------------------------------------------------

// Parsing

impl QueryContinuation {
    pub fn parse(input: &str) -> IResult<&str, QueryContinuation> {
        character::char('&').value(QueryContinuation).parse(input)
    }
}
