#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod string;
mod template;
mod value;

// =============================================================================
// URI Template
// =============================================================================

// Re-Exports

pub use self::{
    template::{
        expand::ExpandError,
        parse::ParseError,
        Template,
    },
    value::{
        Value,
        Values,
    },
};
