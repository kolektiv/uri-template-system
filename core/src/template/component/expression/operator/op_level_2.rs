pub mod fragment;
pub mod reserved;

use crate::template::component::expression::{
    operator::op_level_2::{
        fragment::Fragment,
        reserved::Reserved,
    },
    Behaviour,
};

// =============================================================================
// OpLevel2
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel2<'t> {
    Fragment(Fragment<'t>),
    Reserved(Reserved<'t>),
}

// -----------------------------------------------------------------------------

// Expand

#[rustfmt::skip]
impl<'t> OpLevel2<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        match self {
            Self::Fragment(fragment) => fragment.behaviour(),
            Self::Reserved(reserved) => reserved.behaviour(),
        }
    }
}
