use nom::{
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::{
    expression,
    literal,
    template::{
        Component,
        Template,
    },
};

// =============================================================================
// Parse
// =============================================================================

// Parsers

pub fn template(input: &str) -> IResult<&str, Template> {
    nom::multi::many1(
        literal::parse
            .map(Component::Literal)
            .or(expression::parse.map(Component::Expression)),
    )
    .all_consuming()
    .map(Template)
    .parse(input)
}
