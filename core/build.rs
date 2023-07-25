use std::{
    env,
    fs::{
        self,
        File,
    },
    io::BufReader,
    path::Path,
};

use anyhow::{
    Error,
    Result,
};
use indexmap::IndexMap;
use serde::Deserialize;

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").ok_or_else(|| Error::msg("OUT_DIR not set"))?;
    let dest_path = Path::new(&out_dir).join("expansion.rs");

    println!("dest_path: {dest_path:?}");

    let mut output = String::new();

    for (name, _set) in load_all()? {
        output.push_str(&name);
        output.push_str("\n");

        // for (tpl, exp) in &set.cases {
        // let vars = &set.variables;

        // match exp {
        //     Expansion::String(exp) => {
        //         assert!(exp.eq(tpl), "{name}: {tpl} != {exp}
        // ({vars:#?})")     }
        //     Expansion::List(exps) => {
        //         assert!(exps.contains(tpl), "{name}: {tpl} /∈ {exps:#?}
        // ({vars:#?})")     }
        // }
        // }
    }

    fs::write(&dest_path, &output)?;

    Ok(())
}

// =============================================================================
// Test Cases
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
            let path = format!("cases/official/{name}.json");
            let new_sets = load(path)?;

            all_sets.extend(new_sets.into_iter());

            Ok(all_sets)
        })
}

fn load(path: impl AsRef<Path>) -> Result<IndexMap<String, Set>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let sets = serde_json::from_reader(reader)?;

    Ok(sets)
}
