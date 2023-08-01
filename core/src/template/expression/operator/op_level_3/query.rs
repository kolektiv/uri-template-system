use nom::{
    character::complete as character,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

#[derive(Clone, Debug, PartialEq)]
pub struct Query;

// -----------------------------------------------------------------------------

// Parsing

impl Query {
    pub fn parse(input: &str) -> IResult<&str, Query> {
        character::char('?').value(Query).parse(input)
    }
}
