use uri_template_system_tests::{
    fixtures::{
        self,
        Expansion,
        Group,
    },
    harnesses::{
        self,
        Harness,
    },
};

// =============================================================================
// Expansion
// =============================================================================

// Tests

// Testcases for URI Template processing are generated from the "official" test
// cases published at https://github.com/uri-templates/uritemplate-test, and
// included as a submodule in this repository (./official).

#[test]
fn uri_template_system() {
    test_sets(harnesses::uri_template_system::Harness);
}

#[cfg(feature = "uritemplate-next")]
#[test]
fn uri_template_next() {
    test_sets(harnesses::uri_template_next::Harness);
}

#[cfg(feature = "iri-string")]
#[test]
fn iri_string() {
    test_sets(harnesses::iri_string::Harness);
}

#[rustfmt::skip]
fn test_sets(testable: impl Harness) {
    test_set("Examples", fixtures::examples(), &testable);
    test_set("Examples By Section", fixtures::examples_by_section(), &testable);
    test_set("Extended Tests", fixtures::extended_tests(), &testable);
}

fn test_set(name: &str, groups: Vec<Group>, harness: &impl Harness) {
    for group in groups {
        let name = format!("{name}: {}", group.name);
        let values = harness.prepare(group.variables);

        for (i, case) in group.cases.iter().enumerate() {
            let expansion = &case.expansion;
            let template = &case.template;
            let actual = harness.test(template, &values);

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
