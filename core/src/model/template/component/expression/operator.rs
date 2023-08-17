use crate::{
    model::template::component::expression::{
        Allow,
        Behaviour,
    },
    process::parse::Parse,
};

// =============================================================================
// Operator
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub enum Operator<'t> {
    Level2(OpLevel2<'t>),
    Level3(OpLevel3<'t>),
}
macro_rules! operator {
    ($name:ident) => {
        #[derive(Debug, Eq, PartialEq)]
        pub struct $name<'t> {
            raw: &'t str,
        }

        impl<'t> $name<'t> {
            pub const fn new(raw: &'t str) -> Self {
                Self { raw }
            }
        }
    };
}

// Operator - Level 2

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel2<'t> {
    Fragment(Fragment<'t>),
    Reserved(Reserved<'t>),
}

operator!(Fragment);
operator!(Reserved);

// Operator - Level 3

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel3<'t> {
    Label(Label<'t>),
    Path(Path<'t>),
    PathParameter(PathParameter<'t>),
    Query(Query<'t>),
    QueryContinuation(QueryContinuation<'t>),
}

operator!(Label);
operator!(Path);
operator!(PathParameter);
operator!(Query);
operator!(QueryContinuation);

// -----------------------------------------------------------------------------

// Parse

#[rustfmt::skip]
impl<'t> Parse<'t> for Option<Operator<'t>> {
    fn parse(raw: &'t str) -> (usize, Self) {
        raw.chars().next().and_then(|c| {
            let operator = match c {
                '+' => Some(Operator::Level2(OpLevel2::Reserved(Reserved::new(&raw[..1])))),
                '#' => Some(Operator::Level2(OpLevel2::Fragment(Fragment::new(&raw[..1])))),
                '.' => Some(Operator::Level3(OpLevel3::Label(Label::new(&raw[..1])))),
                '/' => Some(Operator::Level3(OpLevel3::Path(Path::new(&raw[..1])))),
                ';' => Some(Operator::Level3(OpLevel3::PathParameter(PathParameter::new(&raw[..1])))),
                '?' => Some(Operator::Level3(OpLevel3::Query(Query::new(&raw[..1])))),
                '&' => Some(Operator::Level3(OpLevel3::QueryContinuation(QueryContinuation::new(&raw[..1])))),
                _ => None,
            };

            operator.map(|operator| (1, Some(operator)))
        })
        .unwrap_or((0, None))
    }
}

// -----------------------------------------------------------------------------

// Expand

impl<'t> Operator<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        match self {
            Self::Level2(op_level_2) => match op_level_2 {
                OpLevel2::Fragment(_) => &FRAGMENT_BEHAVIOUR,
                OpLevel2::Reserved(_) => &RESERVED_BEHAVIOUR,
            },
            Self::Level3(op_level_3) => match op_level_3 {
                OpLevel3::Label(_) => &LABEL_BEHAVIOUR,
                OpLevel3::Path(_) => &PATH_BEHAVIOUR,
                OpLevel3::PathParameter(_) => &PATH_PARAMETER_BEHAVIOUR,
                OpLevel3::Query(_) => &QUERY_BEHAVIOUR,
                OpLevel3::QueryContinuation(_) => &QUERY_CONTINUATION_BEHAVIOUR,
            },
        }
    }
}

macro_rules! behaviour {
    ($name:ident, $first:stmt, $sep:literal, $named:literal, $ifemp:stmt, $allow:ty) => {
        paste::paste! {
            pub static [< $name:snake:upper _BEHAVIOUR >]: Behaviour = Behaviour {
                first: $first,
                sep: $sep,
                named: $named,
                ifemp: $ifemp,
                allow: $allow
            };
        }
    };
}

// Operator - None

behaviour!(Default, None, ',', false, None, Allow::U);

// Operator - Level 2

behaviour!(Fragment, Some('#'), ',', false, None, Allow::UR);
behaviour!(Reserved, None, ',', false, None, Allow::UR);

// Operator - Level 3

behaviour!(Label, Some('.'), '.', false, None, Allow::U);
behaviour!(Path, Some('/'), '/', false, None, Allow::U);
behaviour!(PathParameter, Some(';'), ';', true, None, Allow::U);
behaviour!(Query, Some('?'), '&', true, Some('='), Allow::U);
behaviour!(QueryContinuation, Some('&'), '&', true, Some('='), Allow::U);
