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
