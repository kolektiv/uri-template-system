mod op_level_2;
mod op_level_3;

use crate::{
    template::component::expression::{
        operator::{
            op_level_2::{
                fragment::Fragment,
                reserved::Reserved,
                OpLevel2,
            },
            op_level_3::{
                label::Label,
                path::Path,
                path_parameter::PathParameter,
                query::Query,
                query_continuation::QueryContinuation,
                OpLevel3,
            },
        },
        Behaviour,
    },
    Parse,
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
            Self::Level2(op_level_2) => op_level_2.behaviour(),
            Self::Level3(op_level_3) => op_level_3.behaviour(),
        }
    }
}
