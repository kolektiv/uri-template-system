use std::{
    fs::OpenOptions,
    io::BufReader,
    path::Path,
};

use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;

// =============================================================================
// Process
// =============================================================================

// Test

// Testcases for URI Template processing are generated from the "official" test
// cases published at https://github.com/uri-templates/uritemplate-test, and
// included as a submodule in this repository (./cases/process).

#[ignore]
#[test]
fn process_test_cases() -> Result<()> {
    for (name, cases) in read_files("tests/cases/process")? {
        for (tpl, exp) in &cases.cases {
            match exp {
                Expansion::List(exps) => list(&name, tpl, exps, &cases.variables),
                Expansion::String(exp) => string(&name, tpl, exp, &cases.variables),
            }
        }
    }

    Ok(())
}

fn list(name: &String, tpl: &String, exps: &Vec<String>, vars: &IndexMap<String, Variable>) {
    assert!(exps.contains(tpl), "{name}: {tpl} ∉ {exps:#?} ({vars:#?})")
}

fn string(name: &String, tpl: &String, exp: &String, vars: &IndexMap<String, Variable>) {
    assert!(exp.eq(tpl), "{name}: {tpl} != {exp} ({vars:#?})")
}

// -----------------------------------------------------------------------------

// Types

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Cases {
    #[serde(default = "default_level")]
    level: u8,
    variables: IndexMap<String, Variable>,
    #[serde(rename = "testcases")]
    cases: Vec<(String, Expansion)>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Variable {
    Simple(String),
    List(Vec<String>),
    AssociativeArray(IndexMap<String, String>),
    Undefined,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Expansion {
    String(String),
    List(Vec<String>),
}

fn default_level() -> u8 {
    4
}

// -----------------------------------------------------------------------------

// Sets

const FILES: &[&str] = &[
    "spec-examples.json",
    "spec-examples-by-section.json",
    // "extended-tests.json",
    // "negative-tests.json",
];

fn read_files(path: impl AsRef<Path>) -> Result<IndexMap<String, Cases>> {
    FILES.iter().try_fold(IndexMap::new(), |mut data, file| {
        let path = path.as_ref().join(file);
        let sets = read_file(path)?;

        data.extend(sets.into_iter());

        Ok(data)
    })
}

fn read_file(path: impl AsRef<Path>) -> Result<IndexMap<String, Cases>> {
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let sets = serde_json::from_reader(reader)?;

    Ok(sets)
}
