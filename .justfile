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

criterion_root := data_home/"criterion"
criterion_home := criterion_root/project_name

bench name *args:
    #!/usr/bin/env bash
    CRITERION_HOME={{criterion_home/name}} cargo bench -p uri-template-system-tests --features {{test_features}} --bench {{name}} -- --verbose {{args}}

# Release

release *args:
    #!/usr/bin/env bash
    cargo release --dependent-version upgrade {{args}}