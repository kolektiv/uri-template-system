use crate::{
    codec::{
        self,
        Encoding,
    },
    expression::Modifier,
    Expand,
};

// =============================================================================
// String
// =============================================================================

// Expansion

impl Expand<String, Encoding> for Option<Modifier> {
    fn expand(&self, output: &mut String, value: &String, context: &Encoding) {
        match self {
            Some(modifier) => modifier.expand(output, value, context),
            _ => codec::encode(value, output, context),
        }
    }
}

impl Expand<String, Encoding> for Modifier {
    fn expand(&self, output: &mut String, value: &String, context: &Encoding) {
        match self {
            Modifier::Explode => codec::encode(value, output, context),
            Modifier::Prefix(max_len) => {
                codec::encode(&value[..(*max_len).min(value.len())], output, context)
            }
        }
    }
}

// None:
//
// Simple: List - v,v,v
//         Hash - k,v,k,v
//
// Explode:
//
// Simple: List - v,v,v
//         Hash - k=v,k=v
// Reserved:
//
