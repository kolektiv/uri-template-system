use nom::{
    character::complete as character,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

// =============================================================================
// Explode
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub struct Explode;

// -----------------------------------------------------------------------------

// Parsing

impl Explode {
    pub fn parse(input: &str) -> IResult<&str, Explode> {
        character::char('*').value(Explode).parse(input)
    }
}
