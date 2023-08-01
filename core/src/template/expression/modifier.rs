mod explode;
mod prefix;

use nom::{
    branch,
    IResult,
    Parser,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    Prefix(Prefix),
    Explode(Explode),
}

impl Modifier {
    pub fn parse(input: &str) -> IResult<&str, Modifier> {
        branch::alt((
            Prefix::parse.map(Modifier::Prefix),
            Explode::parse.map(Modifier::Explode),
        ))
        .parse(input)
    }
}

// -----------------------------------------------------------------------------

// Re-Exports

pub use self::{
    explode::*,
    prefix::*,
};
