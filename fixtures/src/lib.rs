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

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub variables: IndexMap<String, Variable>,
    pub cases: Vec<Case>,
}

#[derive(Debug)]
pub struct Case {
    pub template: String,
    pub expansion: Expansion,
}

#[derive(Debug, Deserialize)]
struct Cases {
    variables: IndexMap<String, Variable>,
    testcases: Vec<(String, Expansion)>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Variable {
    Item(String),
    List(Vec<String>),
    AssociativeArray(IndexMap<String, String>),
    Number(f32),
    Undefined,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Expansion {
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

    serde_json::from_reader::<_, IndexMap<String, Cases>>(BufReader::new(file))
        .expect("deserialization error")
        .into_iter()
        .map(map_case)
        .collect()
}

fn map_case((name, cases): (String, Cases)) -> Group {
    Group {
        name,
        variables: cases.variables,
        cases: cases.testcases.into_iter().map(map_test).collect(),
    }
}

fn map_test((template, expansion): (String, Expansion)) -> Case {
    Case {
        template,
        expansion,
    }
}
