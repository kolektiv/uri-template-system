use std::{
    env,
    fs::OpenOptions,
    io::{
        BufReader,
        BufWriter,
        Write,
    },
    path::Path,
};

use anyhow::{
    Error,
    Result,
};
use indexmap::IndexMap;
use quote::{
    format_ident,
    quote,
};
use serde::Deserialize;

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").ok_or_else(|| Error::msg("OUT_DIR not set"))?;
    let dest_path = Path::new(&out_dir).join("expansion_tests.rs");

    let mut output = String::new();
    let mut index = 0;

    for (name, set) in load_all()? {
        for (template, expansion) in &set.cases {
            let test_name = format_ident!("expansion_{index}");
            let vars = &set.variables;

            match expansion {
                Expansion::String(expansion) => {
                    let err = format!("{name}: {template} not eq to {expansion} (vars: {vars:#?}")
                        .replace("{", "{{")
                        .replace("}", "}}");

                    output.push_str(
                        &quote! {
                            #[test]
                            fn #test_name() {
                                assert!(#expansion.eq(#template), #err);
                            }
                        }
                        .to_string(),
                    );
                }
                Expansion::List(expansions) => {
                    let err = format!("{name}: {template} not in {expansions:#?} (vars: {vars:#?}")
                        .replace("{", "{{")
                        .replace("}", "}}");

                    output.push_str(
                        &quote! {
                            #[test]
                            fn #test_name() {
                                assert!([#(#expansions),*].contains(&#template), #err);
                            }
                        }
                        .to_string(),
                    );
                }
            }

            output.push_str("\n");
            index += 1;
        }
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(dest_path)?;

    let mut writer = BufWriter::new(file);

    writer.write(output.as_bytes())?;

    Ok(())
}

// =============================================================================
// Cases
// =============================================================================

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

// Data

fn load_all() -> Result<IndexMap<String, Set>> {
    ["spec-examples", "spec-examples-by-section"]
        .iter()
        .try_fold(IndexMap::new(), |mut all_sets, name| {
            let path = format!("tests/cases/official/{name}.json");
            let new_sets = load(path)?;

            all_sets.extend(new_sets.into_iter());

            Ok(all_sets)
        })
}

fn load(path: impl AsRef<Path>) -> Result<IndexMap<String, Set>> {
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let sets = serde_json::from_reader(reader)?;

    Ok(sets)
}
