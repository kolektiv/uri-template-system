use std::path::PathBuf;

use uri_template_system_core::{
    URITemplate,
    Value,
    Values,
};
use uri_template_system_fixtures::{
    Case,
    Expansion,
    Group,
    Variable,
};

// =============================================================================
// Expansion
// =============================================================================

// Testcases for URI Template processing are generated from the "official" test
// cases published at https://github.com/uri-templates/uritemplate-test, and
// included as a submodule in this repository (./official).

static FIXTURES_DATA: &str = "../fixtures/data";

#[test]
fn spec_examples() {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples.json");
    let groups = uri_template_system_fixtures::load(path).expect("fixtures error");

    for group in groups {
        assert_group(group);
    }
}

#[test]
fn spec_examples_by_section() {
    let path = PathBuf::from(FIXTURES_DATA).join("spec-examples-by-section.json");
    let groups = uri_template_system_fixtures::load(path).expect("fixtures error");

    for group in groups {
        assert_group(group);
    }
}

#[test]
fn extended_tests() {
    let path = PathBuf::from(FIXTURES_DATA).join("extended-tests.json");
    let groups = uri_template_system_fixtures::load(path).expect("fixtures error");

    for group in groups {
        assert_group(group);
    }
}

fn assert_group(
    Group {
        name,
        variables,
        cases,
    }: Group,
) {
    let values = Values::from_iter(
        variables
            .into_iter()
            .filter_map(|(name, variable)| match variable {
                Variable::AssociativeArray(value) => Some((name, Value::AssociativeArray(value))),
                Variable::Item(value) => Some((name, Value::Item(value))),
                Variable::List(value) => Some((name, Value::List(value))),
                Variable::Number(value) => Some((name, Value::Item(value.to_string()))),
                Variable::Undefined => None,
            })
            .collect::<Vec<_>>(),
    );

    for (
        i,
        Case {
            expansion,
            template,
        },
    ) in cases.iter().enumerate()
    {
        let actual = URITemplate::parse(template)
            .expect(&format!("{name} - {i}: Template Parse Error ({template})"))
            .expand(&values);

        match expansion {
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
