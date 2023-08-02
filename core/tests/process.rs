use std::{
    fs::OpenOptions,
    io::BufReader,
    path::Path,
};

use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use uri_template_system_core as core;

// =============================================================================
// Process
// =============================================================================

// Tests

// Testcases for URI Template processing are generated from the "official" test
// cases published at https://github.com/uri-templates/uritemplate-test, and
// included as a submodule in this repository (./cases/process).

#[test]
fn process_test_cases() -> Result<()> {
    for (name, cases) in get_test_cases("tests/cases/process")? {
        let values = core::Values::from_iter(
            cases
                .values
                .into_iter()
                .filter_map(|(n, v)| match v {
                    Value::AssociativeArray(v) => Some((n, core::Value::AssociativeArray(v))),
                    Value::Item(v) => Some((n, core::Value::Item(v))),
                    Value::List(v) => Some((n, core::Value::List(v))),
                    Value::Number(v) => Some((n, core::Value::Item(v.to_string()))),
                    Value::Undefined => None,
                })
                .collect::<Vec<_>>(),
        );

        for (i, (template, expected)) in cases.cases.iter().enumerate() {
            let actual = core::URITemplate::parse(template)
                .expect(&format!("{name} ({i}): template parse error ({template})"))
                .expand(&values);

            match expected {
                Expansion::List(expected) => {
                    assert!(
                        expected.contains(&actual),
                        "{name} ({i}): \"{actual}\" not in {expected:#?} ({values:#?})"
                    )
                }
                Expansion::String(expected) => {
                    assert!(
                        expected.eq(&actual),
                        "{name} ({i}): \"{actual}\" not equal to \"{expected}\" ({values:#?})"
                    )
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Harness
// =============================================================================

// Types

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Cases {
    #[serde(default = "default_level")]
    level: u8,
    #[serde(rename = "variables")]
    values: IndexMap<String, Value>,
    #[serde(rename = "testcases")]
    cases: Vec<(String, Expansion)>,
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

fn default_level() -> u8 {
    4
}

// -----------------------------------------------------------------------------

// Data

const FILES: &[&str] = &[
    "spec-examples.json",
    "spec-examples-by-section.json",
    "extended-tests.json",
    // "negative-tests.json",
];

fn get_test_cases(path: impl AsRef<Path>) -> Result<IndexMap<String, Cases>> {
    FILES.iter().try_fold(IndexMap::new(), |mut data, file| {
        let path = path.as_ref().join(file);
        let sets = read_test_cases(path)?;

        data.extend(sets.into_iter());

        Ok(data)
    })
}

fn read_test_cases(path: impl AsRef<Path>) -> Result<IndexMap<String, Cases>> {
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let sets = serde_json::from_reader(reader)?;

    Ok(sets)
}
