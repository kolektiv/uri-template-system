mod parse;

use nom::IResult;

// =================================================s============================
// Literal
// =============================================================================

// Types

#[derive(Debug, PartialEq)]
pub struct Literal(String);

impl Literal {
    pub fn new<S>(literal: S) -> Self
    where
        S: Into<String>,
    {
        Self(literal.into())
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::literal(input)
    }
}
