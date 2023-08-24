use crate::template::{
    component::expression::{
        Allow,
        Behaviour,
    },
    Parse,
};

// =============================================================================
// Operator
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
    Level2(OpLevel2),
    Level3(OpLevel3),
}

// Operator - Level 2

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel2 {
    Fragment,
    Reserved,
}

// Operator - Level 3

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel3 {
    Label,
    Path,
    PathParameter,
    Query,
    QueryContinuation,
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> Parse<'t> for Option<Operator> {
    fn parse(raw: &'t str, _global: usize) -> (usize, Self) {
        raw.chars()
            .next()
            .and_then(|c| {
                let operator = match c {
                    '+' => Some(Operator::Level2(OpLevel2::Reserved)),
                    '#' => Some(Operator::Level2(OpLevel2::Fragment)),
                    '.' => Some(Operator::Level3(OpLevel3::Label)),
                    '/' => Some(Operator::Level3(OpLevel3::Path)),
                    ';' => Some(Operator::Level3(OpLevel3::PathParameter)),
                    '?' => Some(Operator::Level3(OpLevel3::Query)),
                    '&' => Some(Operator::Level3(OpLevel3::QueryContinuation)),
                    _ => None,
                };

                operator.map(|operator| (1, Some(operator)))
            })
            .unwrap_or((0, None))
    }
}

// -----------------------------------------------------------------------------

// Expand

impl Operator {
    pub fn behaviour(&self) -> &Behaviour {
        match self {
            Self::Level2(op_level_2) => match op_level_2 {
                OpLevel2::Fragment => &FRAGMENT_BEHAVIOUR,
                OpLevel2::Reserved => &RESERVED_BEHAVIOUR,
            },
            Self::Level3(op_level_3) => match op_level_3 {
                OpLevel3::Label => &LABEL_BEHAVIOUR,
                OpLevel3::Path => &PATH_BEHAVIOUR,
                OpLevel3::PathParameter => &PATH_PARAMETER_BEHAVIOUR,
                OpLevel3::Query => &QUERY_BEHAVIOUR,
                OpLevel3::QueryContinuation => &QUERY_CONTINUATION_BEHAVIOUR,
            },
        }
    }
}

// Operator - None

pub static DEFAULT_BEHAVIOUR: Behaviour = Behaviour {
    first: None,
    sep: ',',
    named: false,
    ifemp: None,
    allow: Allow::U,
};

// Operator - Level 2

pub static FRAGMENT_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('#'),
    sep: ',',
    named: false,
    ifemp: None,
    allow: Allow::UR,
};

pub static RESERVED_BEHAVIOUR: Behaviour = Behaviour {
    first: None,
    sep: ',',
    named: false,
    ifemp: None,
    allow: Allow::UR,
};

// Operator - Level 3

pub static LABEL_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('.'),
    sep: '.',
    named: false,
    ifemp: None,
    allow: Allow::U,
};

pub static PATH_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('/'),
    sep: '/',
    named: false,
    ifemp: None,
    allow: Allow::U,
};

pub static PATH_PARAMETER_BEHAVIOUR: Behaviour = Behaviour {
    first: Some(';'),
    sep: ';',
    named: true,
    ifemp: None,
    allow: Allow::U,
};

pub static QUERY_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('?'),
    sep: '&',
    named: true,
    ifemp: Some('='),
    allow: Allow::U,
};

pub static QUERY_CONTINUATION_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('&'),
    sep: '&',
    named: true,
    ifemp: Some('='),
    allow: Allow::U,
};
