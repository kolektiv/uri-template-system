// mod encode;
mod expand;
mod parse;

// =============================================================================
// Expression
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub struct Expression(Vec<VarSpec>, Option<Operator>);

impl Expression {
    fn new(variable_list: Vec<VarSpec>, operator: Option<Operator>) -> Self {
        Self(variable_list, operator)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct VarSpec(String, Option<Modifier>);

impl VarSpec {
    #[allow(dead_code)]
    pub fn new(var_name: impl Into<String>, modifier: Option<Modifier>) -> Self {
        Self(var_name.into(), modifier)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Modifier {
    Prefix(usize),
    Explode,
}

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Level2(OpLevel2),
    Level3(OpLevel3),
    Reserve(OpReserve),
}

#[derive(Clone, Debug, PartialEq)]
enum OpLevel2 {
    Plus,
    Hash,
}

#[derive(Clone, Debug, PartialEq)]
enum OpLevel3 {
    Period,
    Slash,
    Semicolon,
    Question,
    Ampersand,
}

#[derive(Clone, Debug, PartialEq)]
enum OpReserve {
    Equals,
    Comma,
    Exclamation,
    At,
    Pipe,
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::parse::expression as parse;
