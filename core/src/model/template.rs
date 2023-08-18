mod component;

use std::fmt::{
    self,
    Formatter,
};

use crate::{
    model::{
        template::component::Component,
        value::Values,
    },
    process::{
        expand::{
            Expand,
            Expansion,
        },
        parse::{
            ParseError,
            ParseRef,
            TryParse,
        },
    },
};

// =============================================================================
// Template
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Template<'t> {
    components: Vec<Component<'t>>,
    parse_ref: ParseRef<'t>,
}

impl<'t> Template<'t> {
    #[must_use]
    pub const fn expand<'e>(&'e self, values: &'e Values) -> Expansion<'e, 't> {
        Expansion::new(self, values)
    }

    pub fn parse(raw: &'t str) -> Result<Self, ParseError> {
        Self::try_parse(raw, 0).map(|(_, template)| template)
    }

    const fn new(parse_ref: ParseRef<'t>, components: Vec<Component<'t>>) -> Self {
        Self {
            components,
            parse_ref,
        }
    }
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for Template<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        Vec::<Component<'t>>::try_parse(raw, global).map(|(position, components)| {
            (
                position,
                Self::new(ParseRef::new(0, position - 1, raw), components),
            )
        })
    }
}

// -----------------------------------------------------------------------------

// Expand

impl<'t> Expand for Template<'t> {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> fmt::Result {
        self.components
            .iter()
            .try_for_each(|component| component.expand(values, f))
    }
}
