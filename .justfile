# Environment

data_home := env_var_or_default("XDG_DATA_HOME", "~/.data")
test_features := env_var_or_default("TEST", "default")

# Cargo (Project)

project_name := file_name(justfile_directory())

doc *args:
    #!/usr/bin/env bash
    cargo doc -p uri-template-system --no-deps {{args}}

test *args:
    #!/usr/bin/env bash
    cargo test --features {{test_features}} {{args}}

# Criterion (Benches)

criterion_home := data_home/"criterion"/project_name
criterion_cmd := "cargo bench -p uri-template-system-tests"

compare *args:
    #!/usr/bin/env bash
    CRITERION_HOME={{criterion_home}}/comparison {{criterion_cmd}} --features {{test_features}} --bench comparison -- --verbose {{args}}


optimise *args:
    #!/usr/bin/env bash
    CRITERION_HOME={{criterion_home}}/optimisation {{criterion_cmd}} --bench optimisation -- --verbose {{args}}


# Release

release *args:
    #!/usr/bin/env bash
    cargo release --dependent-version upgrade {{args}}