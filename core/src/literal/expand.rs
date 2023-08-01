use crate::{
    literal::Literal,
    value::Values,
    Expand,
};

// =============================================================================
// Expand
// =============================================================================

impl Expand<Values, ()> for Literal {
    // TODO: Percentage-Encoding/Validation
    fn expand(&self, output: &mut String, _value: &Values, _context: &()) {
        output.push_str(&self.0);
    }
}
