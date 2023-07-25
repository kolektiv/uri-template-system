use std::{
    fs::OpenOptions,
    io::BufReader,
    path::Path,
};

use anyhow::{
    Error,
    Result,
};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use serde::Deserialize;
use syn::Ident;

// =============================================================================
// Process
// =============================================================================

// Generate

pub fn generate(path: impl AsRef<Path>) -> Result<String> {
    let sets = read_files(path)?;
    let tests = tests(sets)?;

    Ok(tests)
}

// -----------------------------------------------------------------------------

// Tests

fn tests(sets: IndexMap<String, Set>) -> Result<String> {
    let mut output = String::new();
    let mut index = 0u32;

    for (set_name, set) in sets {
        for (template, expansion) in &set.cases {
            let test_name = format_ident!("process_{index}");
            let test = match expansion {
                Expansion::String(expansion) => {
                    test_equals(test_name, &set_name, template, expansion, &set.variables)?
                }
                Expansion::List(expansions) => {
                    test_contained(test_name, &set_name, template, expansions, &set.variables)?
                }
            };

            output.push_str(&test);
            output.push_str("\n\n");
            index += 1;
        }
    }

    Ok(output)
}

fn test_equals(
    test_name: Ident,
    set_name: &str,
    template: &str,
    expansion: &str,
    variables: &IndexMap<String, Variable>,
) -> Result<String> {
    let err = format!("{set_name}: {template} not eq to {expansion} (vars: {variables:#?}")
        .replace("{", "{{")
        .replace("}", "}}");

    format(quote! {
        #[test]
        fn #test_name() {
            assert!(#expansion.eq(#template), #err);
        }
    })
}

fn test_contained(
    test_name: Ident,
    set_name: &str,
    template: &str,
    expansions: &Vec<String>,
    variables: &IndexMap<String, Variable>,
) -> Result<String> {
    let err = format!("{set_name}: {template} not in {expansions:#?} (vars: {variables:#?}")
        .replace("{", "{{")
        .replace("}", "}}");

    format(quote! {
        #[test]
        fn #test_name() {
            assert!([#(#expansions),*].contains(&#template), #err);
        }
    })
}

fn format(tokens: TokenStream) -> Result<String> {
    syn::parse_file(&tokens.to_string())
        .map(|syntax| prettyplease::unparse(&syntax))
        .map_err(Error::from)
}

// -----------------------------------------------------------------------------

// Types

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Set {
    level: Option<u8>,
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
    // False,
}

// -----------------------------------------------------------------------------

// Sets

const FILES: &[&str] = &["spec-examples.json", "spec-examples-by-section.json"];

fn read_files(path: impl AsRef<Path>) -> Result<IndexMap<String, Set>> {
    FILES.iter().try_fold(IndexMap::new(), |mut data, file| {
        let path = path.as_ref().join(file);
        let sets = read_file(path)?;

        data.extend(sets.into_iter());

        Ok(data)
    })
}

fn read_file(path: impl AsRef<Path>) -> Result<IndexMap<String, Set>> {
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let sets = serde_json::from_reader(reader)?;

    Ok(sets)
}
