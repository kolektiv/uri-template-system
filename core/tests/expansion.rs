use uri_template_system_fixtures::{
    self as fixtures,
    Expansion,
    Group,
};

// =============================================================================
// Expansion
// =============================================================================

// Tests

// Testcases for URI Template processing are generated from the "official" test
// cases published at https://github.com/uri-templates/uritemplate-test, and
// included as a submodule in this repository (./official).

#[test]
fn examples() {
    test("Examples", fixtures::examples());
}

#[test]
fn examples_by_section() {
    test("Examples By Section", fixtures::examples_by_section());
}

#[test]
fn extended_tests() {
    test("Extended Tests", fixtures::extended_tests());
}

// -----------------------------------------------------------------------------

// Test

fn test(name: &str, groups: Vec<Group>) {
    for group in groups {
        let name = format!("{name}: {}", group.name);
        let values = uri_template_system::prepare(group.variables);

        for (i, case) in group.cases.iter().enumerate() {
            let expansion = &case.expansion;
            let template = &case.template;
            let actual = uri_template_system::test(template, &values);

            match expansion {
                Expansion::Multiple(expected) => {
                    assert!(
                        expected.contains(&actual),
                        "{name} - {i}: Actual expansion \"{actual}\" not found in expected \
                         expansions {expected:#?}.\nTemplate: \"{template}\"\nValues: {values:#?}"
                    )
                }
                Expansion::Single(expected) => {
                    assert!(
                        expected.eq(&actual),
                        "{name} - {i}: Actual expansion \"{actual}\" not equal to expected \
                         expansion \"{expected}\".\nTemplate: \"{template}\"\nValues: {values:#?}"
                    )
                }
            }
        }
    }
}

// =============================================================================
// Implementation
// =============================================================================

// URI Template System

mod uri_template_system {
    use uri_template_system_core::{
        URITemplate,
        Value,
        Values,
    };
    use uri_template_system_fixtures::Variable;

    pub fn prepare(variables: Vec<(String, Variable)>) -> Values {
        Values::from_iter(variables.into_iter().map(|(n, v)| match v {
            Variable::AssociativeArray(v) => (n, Value::AssociativeArray(v)),
            Variable::Item(v) => (n, Value::Item(v)),
            Variable::List(v) => (n, Value::List(v)),
        }))
    }

    pub fn test(template: &str, values: &Values) -> String {
        URITemplate::parse(template)
            .unwrap()
            .expand(values)
            .to_string()
    }
}
