# uri-template-system

[<img alt="GitHub" src="https://img.shields.io/badge/github-code-999999?style=for-the-badge&logo=github" height="20">](https://github.com/kolektiv/uri-template-system) [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-documentation-999999?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/uri-template-system) [<img alt="Crates" src="https://img.shields.io/crates/v/uri-template-system?style=for-the-badge&logo=rust" height="20">](https://crates.io/crates/uri-template-system) [<img alt="Continuous Integration" src="https://img.shields.io/github/actions/workflow/status/kolektiv/uri-template-system/ci.yml?style=for-the-badge&logo=github" height="20">](https://github.com/kolektiv/uri-template-system/actions/workflows/ci.yml) [<img alt="Issues" src="https://img.shields.io/github/issues/kolektiv/uri-template-system?style=for-the-badge&logo=github" height="20">](https://github.com/kolektiv/uri-template-system/issues) [<img alt="Sponsors" src="https://img.shields.io/github/sponsors/kolektiv?style=for-the-badge&logo=github" height="20">](https://github.com/kolektiv)

URI Templates [(RFC6570)](https://datatracker.ietf.org/doc/html/rfc6570) are an underrated tool for web programming. They regularise the construction of URIs and related forms, removing the need for ad-hoc string manipulation, and can help provide consistency across complex web applications.

```rust
use uri_template_system_core::{ Template, Value, Values };

let template = Template::parse("/hello/{name}/from{/library*}").unwrap();
let values = Values::default()
    .add("name", Value::item("world"))
    .add("library", Value::list(["uri", "template", "system"]));

assert_eq!(
    template.expand(&values).unwrap(),
    "/hello/world/from/uri/template/system"
);
```

Beyond the scope of the RFC itself, they can also be used (with some judgement) for matching URIs and related forms as well, effectively providing the basis for a routing mechanism which can be driven from the same data as the linking mechanism. No more mismatches between linking and routing.

## Scope

This library provides an implementation of URI Templates which complies with the RFC and passes the standard test cases for parsing and expansion.

Future iterations will extend this to provide matching for both single and multiple URI templates, routing based on URI Template matching, and support for strongly-typed template data. See the [milestones](https://github.com/kolektiv/uri-template-system/milestones) for the latest progress and for more detailed descriptions of planned features.

## Goals

Existing implementations of URI Templates exist for Rust, notably [`rust-uritemplate`](https://github.com/chowdhurya/rust-uritemplate) and the various forks of that implementation, and an implementation in the broader [`iri-string`](https://github.com/lo48576/iri-string) library.

This implementation aims to be both more "correct" (the parser is intentionally strict with regards to the RFC, while the existing implementations allow some malformed templates through as valid) and more structured in terms of underlying representation (with the aim of making additional features such as matching more tractable). Performance is also important, and the implementation is roughly on-par with `iri-string` (the `rust-uritemplate` derivations have a slightly different programming model, but even allowing for this, they are generally significantly slower).