// =============================================================================
// Process
// =============================================================================

// Tests for URI Template processing are generated from the "official" test
// cases published at https://github.com/uri-templates/uritemplate-test, and
// included as a submodule in this repository (./cases/process).

// Tests are generated for each individual case given - see build.rs and related
// modules for details.

include!(concat!(env!("OUT_DIR"), "/process_gen.rs"));
