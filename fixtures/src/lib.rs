use std::{
    fs::OpenOptions,
    io::BufReader,
    path::Path,
};

use indexmap::IndexMap;
use serde::Deserialize;

// =============================================================================
// Fixtures
// =============================================================================

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

// Functions

pub fn load(path: impl AsRef<Path>) -> Vec<Group> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("file open error");

    serde_json::from_reader::<_, IndexMap<String, JSONCases>>(BufReader::new(file))
        .expect("deserialization error")
        .into_iter()
        .map(|(name, cases)| Group {
            name,
            variables: cases
                .variables
                .into_iter()
                .filter_map(|(name, variable)| match variable {
                    JSONVariable::AssociativeArray(value) => Some((
                        name,
                        Variable::AssociativeArray(value.into_iter().collect()),
                    )),
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
