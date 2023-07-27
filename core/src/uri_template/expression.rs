mod parse;

use nom::IResult;

// =============================================================================
// Expression
// =============================================================================

// Types

#[derive(Debug, PartialEq)]
pub struct Expression(Vec<VarSpec>, Option<Operator>);

impl Expression {
    pub fn new(variable_list: Vec<VarSpec>, operator: Option<Operator>) -> Self {
        Self(variable_list, operator)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::expression(input)
    }
}

#[derive(Debug, PartialEq)]
pub struct VarSpec(String, Option<Modifier>);

impl VarSpec {
    pub fn new<S>(varname: S, modifier: Option<Modifier>) -> Self
    where
        S: Into<String>,
    {
        Self(varname.into(), modifier)
    }
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Prefix(usize),
    Explode,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Level2(OpLevel2),
    Level3(OpLevel3),
    Reserve(OpReserve),
}

#[derive(Debug, PartialEq)]
pub enum OpLevel2 {
    Plus,
    Hash,
}

#[derive(Debug, PartialEq)]
pub enum OpLevel3 {
    Period,
    Slash,
    Semicolon,
    Question,
    Ampersand,
}

#[derive(Debug, PartialEq)]
pub enum OpReserve {
    Equals,
    Comma,
    Exclamation,
    At,
    Pipe,
}
