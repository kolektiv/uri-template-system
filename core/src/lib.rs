mod encode;
mod expand;
mod parse;
mod satisfy;

use std::collections::HashMap;

use anyhow::Result;
use fnv::FnvBuildHasher;

use crate::{
    expand::Expansion,
    parse::TryParse,
};

// =============================================================================
// URI Template
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct URITemplate<'t> {
    template: Template<'t>,
}

impl<'t> URITemplate<'t> {
    pub fn expand<'e>(&'e self, values: &'e Values) -> Expansion<'e, 't> {
        Expansion::new(&self.template, values)
    }

    pub fn parse(raw: &'t str) -> Result<Self> {
        Template::try_parse(raw).map(|(_, template)| Self { template })
    }
}

// -----------------------------------------------------------------------------

// Template

#[derive(Debug, Eq, PartialEq)]
pub struct Template<'t> {
    pub components: Vec<Component<'t>>,
    pub raw: &'t str,
}

impl<'t> Template<'t> {
    const fn new(raw: &'t str, components: Vec<Component<'t>>) -> Self {
        Self { components, raw }
    }
}

// -----------------------------------------------------------------------------

// Component

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'t> {
    Literal(Literal<'t>),
    Expression(Expression<'t>),
}

// -----------------------------------------------------------------------------

// Expression

#[derive(Debug, Eq, PartialEq)]
pub struct Expression<'t> {
    operator: Option<Operator<'t>>,
    raw: &'t str,
    variable_list: VariableList<'t>,
}

impl<'t> Expression<'t> {
    const fn new(
        raw: &'t str,
        operator: Option<Operator<'t>>,
        variable_list: VariableList<'t>,
    ) -> Self {
        Self {
            operator,
            raw,
            variable_list,
        }
    }
}

// -----------------------------------------------------------------------------

// Operator

#[derive(Debug, Eq, PartialEq)]
pub enum Operator<'t> {
    Level2(OpLevel2<'t>),
    Level3(OpLevel3<'t>),
}
macro_rules! operator {
    ($name:ident) => {
        #[derive(Debug, Eq, PartialEq)]
        pub struct $name<'t> {
            raw: &'t str,
        }

        impl<'t> $name<'t> {
            pub const fn new(raw: &'t str) -> Self {
                Self { raw }
            }
        }
    };
}

// Operator - Level 2

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel2<'t> {
    Fragment(Fragment<'t>),
    Reserved(Reserved<'t>),
}

operator!(Fragment);
operator!(Reserved);

// Operator - Level 3

#[derive(Debug, Eq, PartialEq)]
pub enum OpLevel3<'t> {
    Label(Label<'t>),
    Path(Path<'t>),
    PathParameter(PathParameter<'t>),
    Query(Query<'t>),
    QueryContinuation(QueryContinuation<'t>),
}

operator!(Label);
operator!(Path);
operator!(PathParameter);
operator!(Query);
operator!(QueryContinuation);

// -----------------------------------------------------------------------------

// Variable - List

pub type VariableList<'t> = Vec<VariableSpecification<'t>>;

// Variable - Specification

pub type VariableSpecification<'t> = (VariableName<'t>, Option<Modifier<'t>>);

// Variable - Name

#[derive(Debug, Eq, PartialEq)]
pub struct VariableName<'t> {
    raw: &'t str,
}

impl<'t> VariableName<'t> {
    const fn new(raw: &'t str) -> Self {
        Self { raw }
    }

    pub fn name(&self) -> &str {
        self.raw
    }
}

// -----------------------------------------------------------------------------

// Modifier

#[derive(Debug, Eq, PartialEq)]
pub enum Modifier<'t> {
    Explode(Explode<'t>),
    Prefix(Prefix<'t>),
}

// Modifier - Explode

operator!(Explode);

// Modifier - Prefix

#[derive(Debug, Eq, PartialEq)]
pub struct Prefix<'t> {
    length: usize,
    raw: &'t str,
}

impl<'t> Prefix<'t> {
    pub fn new(raw: &'t str, length: usize) -> Self {
        Self { length, raw }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

// -----------------------------------------------------------------------------

// Literal

#[derive(Debug, Eq, PartialEq)]
pub struct Literal<'t> {
    raw: &'t str,
}

impl<'t> Literal<'t> {
    pub const fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// -----------------------------------------------------------------------------

// Values

#[derive(Clone, Debug)]
pub struct Values {
    pub values: HashMap<String, Value, FnvBuildHasher>,
}

impl Values {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }
}

impl FromIterator<(String, Value)> for Values {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Self {
            values: HashMap::from_iter(iter),
        }
    }
}

// Value

#[derive(Clone, Debug)]
pub enum Value {
    AssociativeArray(Vec<(String, String)>),
    Item(String),
    List(Vec<String>),
}

impl Value {
    pub fn defined(&self) -> bool {
        match self {
            Self::AssociativeArray(value) if value.is_empty() => false,
            Self::List(value) if value.is_empty() => false,
            _ => true,
        }
    }
}
