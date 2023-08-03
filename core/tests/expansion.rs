use std::path::PathBuf;

use indexmap::IndexMap;
use uri_template_system_core::{
    URITemplate,
    Value,
    Values,
};
use uri_template_system_fixtures::{
    Expansion,
    Group,
    Variable,
};

// =============================================================================
// Expansion
// =============================================================================

// Tests

// Testcases for URI Template processing are generated from the "official" test
// cases published at https://github.com/uri-templates/uritemplate-test, and
// included as a submodule in this repository (./official).

static FIXTURES_DATA: &str = "../fixtures/data";

#[test]
fn spec_examples() {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples.json");
    let groups = uri_template_system_fixtures::load(path);

    for group in groups {
        test(group);
    }
}

#[test]
fn spec_examples_by_section() {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples-by-section.json");
    let groups = uri_template_system_fixtures::load(path);

    for group in groups {
        test(group);
    }
}

#[test]
fn extended_tests() {
    let path = PathBuf::from(FIXTURES_DATA).join("extended-tests.json");
    let groups = uri_template_system_fixtures::load(path);

    for group in groups {
        test(group);
    }
}

// -----------------------------------------------------------------------------

// Test

fn test(group: Group) {
    let name = &group.name;
    let values = to_values(group.variables);

    for (i, case) in group.cases.iter().enumerate() {
        let expansion = &case.expansion;
        let template = &case.template;

        let actual = URITemplate::parse(template)
            .expect(&format!("{name} - {i}: Template Parse Error ({template})"))
            .expand(&values);

        match expansion {
            Expansion::Multiple(expected) => {
                assert!(
                    expected.contains(&actual),
                    "{name} - {i}: Actual expansion \"{actual}\" not found in expected expansions \
                     {expected:#?}.\nTemplate: \"{template}\"\nValues: {values:#?}"
                )
            }
            Expansion::Single(expected) => {
                assert!(
                    expected.eq(&actual),
                    "{name} - {i}: Actual expansion \"{actual}\" not equal to expected expansion \
                     \"{expected}\".\nTemplate: \"{template}\"\nValues: {values:#?}"
                )
            }
        }
    }
}

fn to_values(variables: Vec<(String, Variable)>) -> Values {
    Values::from_iter(variables.into_iter().map(to_value))
}

fn to_value((n, v): (String, Variable)) -> (String, Value) {
    match v {
        Variable::AssociativeArray(v) => (n, Value::AssociativeArray(IndexMap::from_iter(v))),
        Variable::Item(v) => (n, Value::Item(v)),
        Variable::List(v) => (n, Value::List(v)),
    }
}
