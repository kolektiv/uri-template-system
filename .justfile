# Cargo (Project)

project_name := file_name(justfile_directory())

test *args:
    #!/usr/bin/env bash
    cargo test {{args}}

# Criterion (Benches)

criterion_home_root := "~/.data/criterion"
criterion_home := criterion_home_root/project_name

bench name *args:
    #!/usr/bin/env bash
    CRITERION_HOME={{criterion_home/name}} cargo bench --bench {{name}} -- --verbose {{args}}