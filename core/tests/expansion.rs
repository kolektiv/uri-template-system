use std::{
    fs::OpenOptions,
    io::BufReader,
    path::{
        Path,
        PathBuf,
    },
};

use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use uri_template_system_core as core;

// =============================================================================
// Expansion
// =============================================================================

// Tests

// Testcases for URI Template processing are generated from the "official" test
// cases published at https://github.com/uri-templates/uritemplate-test, and
// included as a submodule in this repository (./official).

static ROOT_PATH: &str = "tests/official";

#[test]
fn spec_examples() {
    let path = PathBuf::from(ROOT_PATH).join("spec-examples.json");
    let cases = read_cases(path).expect("failed to read test file");

    for (name, cases) in cases {
        assert_tests(name, cases.tests, convert_values(cases.values));
    }
}

#[test]
fn spec_examples_by_section() {
    let path = PathBuf::from(ROOT_PATH).join("spec-examples-by-section.json");
    let cases = read_cases(path).expect("failed to read test file");

    for (name, cases) in cases {
        assert_tests(name, cases.tests, convert_values(cases.values));
    }
}

#[test]
fn extended_tests() {
    let path = PathBuf::from(ROOT_PATH).join("extended-tests.json");
    let cases = read_cases(path).expect("failed to read test file");

    for (name, cases) in cases {
        assert_tests(name, cases.tests, convert_values(cases.values));
    }
}

// =============================================================================
// Harness
// =============================================================================

// Types

#[derive(Debug, Deserialize)]
struct Cases {
    #[serde(rename = "variables")]
    values: IndexMap<String, Value>,
    #[serde(rename = "testcases")]
    tests: Vec<(String, Expansion)>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Value {
    Item(String),
    List(Vec<String>),
    AssociativeArray(IndexMap<String, String>),
    Number(f32),
    Undefined,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Expansion {
    String(String),
    List(Vec<String>),
}

// -----------------------------------------------------------------------------

// Data

fn read_cases(path: impl AsRef<Path>) -> Result<IndexMap<String, Cases>> {
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let sets = serde_json::from_reader(reader)?;

    Ok(sets)
}

fn convert_values(values: IndexMap<String, Value>) -> core::Values {
    core::Values::from_iter(
        values
            .into_iter()
            .filter_map(|(n, v)| match v {
                Value::AssociativeArray(v) => Some((n, core::Value::AssociativeArray(v))),
                Value::Item(v) => Some((n, core::Value::Item(v))),
                Value::List(v) => Some((n, core::Value::List(v))),
                Value::Number(v) => Some((n, core::Value::Item(v.to_string()))),
                Value::Undefined => None,
            })
            .collect::<Vec<_>>(),
    )
}

fn assert_tests(name: String, cases: Vec<(String, Expansion)>, values: core::Values) {
    for (i, (template, expected)) in cases.iter().enumerate() {
        let actual = core::URITemplate::parse(template)
            .expect(&format!("{name} - {i}: Template Parse Error ({template})"))
            .expand(&values);

        match expected {
            Expansion::List(expected) => {
                assert!(
                    expected.contains(&actual),
                    "{name} - {i}: Actual expansion \"{actual}\" not found in expected expansions \
                     {expected:#?}.\nTemplate: \"{template}\"\nValues: {values:#?}"
                )
            }
            Expansion::String(expected) => {
                assert!(
                    expected.eq(&actual),
                    "{name} - {i}: Actual expansion \"{actual}\" not equal to expected expansion \
                     \"{expected}\".\nTemplate: \"{template}\"\nValues: {values:#?}"
                )
            }
        }
    }
}
