# Opinionated Rust Template
A template for your next Rust project.

To start coding run:
```bash
cargo generate tomkarw/opinionated-rust-template
```

Only requires `just` to bootstrap all tools and configuration:
```bash
cargo install just
```

Includes:
- quite strict `clippy` warnings; some lints are explicitly denied (those that have easy idiomatic alternatives).
- opinionated `rustfmt` configuration: separate imports by std/external/crate,
    import granularity at module level and formatting string literals
- `just init` for installing all necessary tools
- pre-commit hooks for `cargo {fmt,clippy,test}`
- `just pre-commit` command that additionally includes a spellcheck
-  GitHub CI pipeline taken from [hyper project pipeline]
- skeleton for integration and e2e tests
- skeleton for benchmarks with `criterion`
- MIT and Apache 2.0 licenses

## Description of included things
- [Conventional Commit] - a specification for adding human and machine-readable meaning to commit messages
- [just] - handy way to save and run project-specific commands
- [pre-commit] - framework for managing and maintaining multi-language pre-commit hooks
- [cargo-nextest] - next-generation test runner
- [cargo-spellcheck] - check your spelling with `hunspell` and/or `nlprule`
- [cargo-llvm-cov] - easily use LLVM source-based code coverage
- [criterion] - helps you write fast code by detecting and measuring performance improvements or regressions
- [cargo-udeps] - find unused dependencies in `Cargo.toml`
- [cargo-audit] - audit `Cargo.lock` files for crates with security vulnerabilities reported to the [RustSec Advisory Database]
- [cargo-upgrades] - shows which dependencies in `Cargo.toml` can be upgraded to a newer version
- [cargo-unused-features]- allows you to find and prune enabled, but, potentially unused feature flags from your project

## Other recommended tools
- [cargo-hack] - test each feature independently and all combinations of feature powerset
- [cargo-modules] - plugin for showing an overview of a crate's modules
- [cargo-geiger] - tool that lists statistics related to the usage of unsafe Rust code in a Rust crate and all its dependencies

## TODO
- [x] template project to provide `--bin` and `--lib` versions in `cargo-generate`
- [x] add all commands from `just thorough-check` to CI
- [x] add choice of FOSS licences (MIT/APACHE/both/neither)
- [x] add comments to all of `just` commands
- [ ] add confirmation for each of steps performed in `just thorough-check`
- [ ] extend `pre-commit-rust` with all things in `just pre-commit`
- [ ] investigate `cargo-sync-readme`
- [ ] investigate `rust-semverver/cargo-public-api/cargo-semver-checks`
- [ ] meta: speed up build-template CI run

## Similar projects

You can find similar templates via [cargo-generate topic on GitHub]

## License

This project is licensed under either of:
* Apache License, Version 2.0, ([LICENSE-APACHE] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Authors

Tomasz Karwowski
[LinkedIn]

[//]: # (general links)
[hyper project pipeline]: https://github.com/hyperium/hyper/blob/master/.github/workflows/CI.yml

[just]: https://github.com/casey/just#readme
[Conventional Commit]: https://www.conventionalcommits.org/en/v1.0.0/
[pre-commit]: https://pre-commit.com/
[cargo-nextest]: https://github.com/nextest-rs/nextest#readme
[cargo-spellcheck]: https://github.com/drahnr/cargo-spellcheck#readme
[cargo-llvm-cov]: https://github.com/taiki-e/cargo-llvm-cov
[criterion]: https://github.com/bheisler/criterion.rs#quickstart
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-audit]: https://github.com/rustsec/rustsec/tree/HEAD/cargo-audit#readme
[RustSec Advisory Database]: https://github.com/RustSec/advisory-db/#readme
[cargo-upgrades]: https://gitlab.com/kornelski/cargo-upgrades/-/blob/main/README.md
[cargo-unused-features]: https://github.com/timonpost/cargo-unused-features#readme

[cargo-hack]: https://github.com/taiki-e/cargo-hack/#readme
[cargo-modules]: https://github.com/regexident/cargo-modules#readme
[cargo-geiger]: https://github.com/rust-secure-code/cargo-geiger/tree/HEAD/cargo-geiger#readme

[cargo-generate topic on GitHub]: https://github.com/topics/cargo-generate

[LICENSE-APACHE]: ./LICENSE-APACHE
[LICENSE-MIT]: ./LICENSE-MIT

[LinkedIn]: https://www.linkedin.com/in/tomek-karwowski/
