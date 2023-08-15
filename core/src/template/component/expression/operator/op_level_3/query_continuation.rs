use crate::template::component::expression::{
    Allow,
    Behaviour,
};

// =============================================================================
// Query Continuation
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct QueryContinuation<'t> {
    raw: &'t str,
}

impl<'t> QueryContinuation<'t> {
    pub fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Expand

static QUERY_CONTINUATION_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('&'),
    sep: '&',
    named: true,
    ifemp: Some('='),
    allow: Allow::Unreserved,
};

impl<'t> QueryContinuation<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        &QUERY_CONTINUATION_BEHAVIOUR
    }
}
