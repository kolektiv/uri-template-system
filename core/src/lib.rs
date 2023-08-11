mod codec;

mod template;
mod value;

use anyhow::Result;
use fnv::FnvBuildHasher;

use crate::template::Template;

// =============================================================================
// URI Template
// =============================================================================

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(raw: &'a str) -> Result<(usize, Self)>;
}

// #[derive(Debug, Eq, PartialEq)]
// pub struct ParseRef<'a> {
//     start: usize,
//     end: usize,
//     slice: &'a str,
// }

// impl<'a> ParseRef<'a> {
//     pub fn new(start: usize, end: usize, slice: &'a str) -> Self {
//         Self { start, end, slice }
//     }
// }

// Types

// TODO: Don't leak this implementation detail

pub type IndexMap<K, V> = indexmap::IndexMap<K, V, FnvBuildHasher>;

// -----------------------------------------------------------------------------

// Traits

trait Expand<V, C> {
    fn expand(&self, output: &mut String, value: &V, context: &C);
}

// -----------------------------------------------------------------------------

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct URITemplate<'a> {
    template: Template<'a>,
}

impl<'a> URITemplate<'a> {
    pub fn parse(raw: &'a str) -> Result<Self> {
        Template::parse(raw).map(|(_, template)| Self { template })
    }

    pub fn expand(&self, values: &Values) -> String {
        let mut output = String::new();
        let values = values.defined();

        self.template.expand(&mut output, &values, &());

        output
    }
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::value::{
    Value,
    Values,
};
