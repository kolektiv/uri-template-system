mod action {
    pub mod encode;
    pub mod expand;
    pub mod parse;
    pub mod satisfy;
}

mod model {
    pub mod template;
    pub mod value;
}

// =============================================================================
// URI Template
// =============================================================================

// Re-Exports

pub use crate::{
    action::expand::Expansion,
    model::{
        template::Template,
        value::{
            Value,
            Values,
        },
    },
};
