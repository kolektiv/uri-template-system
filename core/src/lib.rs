#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![allow(clippy::missing_errors_doc)] // TODO: Remove

mod model {
    pub mod template;
    pub mod value;
}

mod process {
    pub mod expand;
    pub mod parse;
}

mod util {
    pub mod encode;
    pub mod satisfy;
}

// =============================================================================
// URI Template
// =============================================================================

// Re-Exports

pub use crate::{
    model::{
        template::Template,
        value::{
            Value,
            Values,
        },
    },
    process::expand::Expansion,
};
