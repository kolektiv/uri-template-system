use anyhow::Result;

use crate::{
    literal::Literal,
    value::Values,
    Expand,
};

// =============================================================================
// Expand
// =============================================================================

impl Expand for Literal {
    type Context = ();

    // TODO: Percentage-Encoding/Validation
    fn expand(&self, output: &mut String, _: &Values, _: &Self::Context) -> Result<()> {
        output.push_str(&self.0);

        Ok(())
    }
}
