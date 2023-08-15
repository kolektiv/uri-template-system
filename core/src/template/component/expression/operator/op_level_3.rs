pub mod label;
pub mod path;
pub mod path_parameter;
pub mod query;
pub mod query_continuation;

use crate::template::component::expression::{
    operator::op_level_3::{
        label::Label,
        path::Path,
        path_parameter::PathParameter,
        query::Query,
        query_continuation::QueryContinuation,
    },
    Behaviour,
};

// =============================================================================
// OpLevel3
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel3<'t> {
    Label(Label<'t>),
    Path(Path<'t>),
    PathParameter(PathParameter<'t>),
    Query(Query<'t>),
    QueryContinuation(QueryContinuation<'t>),
}

// -----------------------------------------------------------------------------

// Expand

impl<'t> OpLevel3<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        match self {
            Self::Label(label) => label.behaviour(),
            Self::Path(path) => path.behaviour(),
            Self::PathParameter(path_parameter) => path_parameter.behaviour(),
            Self::Query(query) => query.behaviour(),
            Self::QueryContinuation(query_continuation) => query_continuation.behaviour(),
        }
    }
}
