mod build {
    pub mod tests {
        pub mod process;
    }
}

use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::Path,
};

use anyhow::{
    Error,
    Result,
};

use self::build::tests::process;

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").ok_or_else(|| Error::msg("OUT_DIR not set"))?;

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(Path::new(&out_dir).join("process_gen.rs"))?
        .write(process::generate("tests/cases/process")?.as_bytes())?;

    Ok(())
}
