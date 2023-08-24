use std::sync::OnceLock;

use indexmap::IndexMap;
use serde::Deserialize;

// =============================================================================
// Fixtures
// =============================================================================

#[must_use]
pub fn examples() -> Vec<Group> {
    data().0.clone()
}

#[must_use]
pub fn examples_by_section() -> Vec<Group> {
    data().1.clone()
}

#[must_use]
pub fn extended_tests() -> Vec<Group> {
    data().2.clone()
}

// -----------------------------------------------------------------------------

// Types

#[derive(Clone, Debug)]
pub struct Group {
    pub name: String,
    pub variables: Vec<(String, Variable)>,
    pub cases: Vec<Case>,
}

#[derive(Clone, Debug)]
pub struct Case {
    pub template: String,
    pub expansion: Expansion,
}

#[derive(Clone, Debug)]
pub enum Variable {
    AssociativeArray(Vec<(String, String)>),
    Item(String),
    List(Vec<String>),
}

#[derive(Clone, Debug)]
pub enum Expansion {
    Single(String),
    Multiple(Vec<String>),
}

// -----------------------------------------------------------------------------

// JSON Types

#[derive(Debug, Deserialize)]
struct JSONCases {
    variables: IndexMap<String, JSONVariable>,
    testcases: Vec<(String, JSONExpansion)>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum JSONVariable {
    Item(String),
    List(Vec<String>),
    AssociativeArray(IndexMap<String, String>),
    Number(f32),
    Undefined,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum JSONExpansion {
    String(String),
    List(Vec<String>),
}

// -----------------------------------------------------------------------------

// Data

static DATA: OnceLock<(Vec<Group>, Vec<Group>, Vec<Group>)> = OnceLock::new();

fn data() -> &'static (Vec<Group>, Vec<Group>, Vec<Group>) {
    DATA.get_or_init(|| {
        (
            load(include_str!("../data/spec-examples.json")),
            load(include_str!("../data/spec-examples-by-section.json")),
            load(include_str!("../data/extended-tests.json")),
        )
    })
}

#[rustfmt::skip]
fn load(fixtures_json: &str) -> Vec<Group> {
    serde_json::from_str::<IndexMap<String, JSONCases>>(fixtures_json)
        .expect("deserialization error")
        .into_iter()
        .map(|(name, cases)| Group {
            name,
            variables: cases
                .variables
                .into_iter()
                .filter_map(|(name, variable)| match variable {
                    JSONVariable::AssociativeArray(value) => Some((name, Variable::AssociativeArray(value.into_iter().collect()))),
                    JSONVariable::Item(value) => Some((name, Variable::Item(value))),
                    JSONVariable::List(value) => Some((name, Variable::List(value))),
                    JSONVariable::Number(value) => Some((name, Variable::Item(value.to_string()))),
                    JSONVariable::Undefined => None,
                })
                .collect(),
            cases: cases
                .testcases
                .into_iter()
                .map(|(template, expansion)| Case {
                    template,
                    expansion: match expansion {
                        JSONExpansion::List(expansion) => Expansion::Multiple(expansion),
                        JSONExpansion::String(expansion) => Expansion::Single(expansion),
                    },
                })
                .collect(),
        })
        .collect()
}
