# Cargo (Project)

project_name := file_name(justfile_directory())

doc *args:
    #!/usr/bin/env bash
    cargo doc -p uri-template-system --no-deps {{args}}

test *args:
    #!/usr/bin/env bash
    cargo test {{args}}

# Criterion (Benches)

criterion_home_root := "~/.data/criterion"
criterion_home := criterion_home_root/project_name

bench name *args:
    #!/usr/bin/env bash
    CRITERION_HOME={{criterion_home/name}} cargo bench -p uri-template-system-tests --features $COMPARE --bench {{name}} -- --verbose {{args}}

# Release

release *args:
    #!/usr/bin/env bash
    cargo release --dependent-version upgrade {{args}}