#!/usr/bin/env just --justfile
set dotenv-load := true

list:
    @just --list

help:
    @just list

fmt *FLAGS:
    cargo +nightly fmt {{FLAGS}}

test *FLAGS:
    cargo nextest run {{FLAGS}}

pre-commit:
    @just fmt
    cargo spellcheck fix
    cargo clippy
    @just test

coverage:
    cargo llvm-cov --open

benchmark:
    cargo criterion

thorough-check:
    cargo +nightly udeps
    cargo audit
    cargo upgrades
    @just unused-features

unused-features:
    unused-features analyze
    unused-features build-report --input report.json
    rm report.json
    mv report.html /tmp
    xdg-open /tmp/report.html

init:
    echo # installing git hooks
    pip install pre-commit
    pre-commit install
    echo # installing nightly used by `just fmt` and `cargo udeps`
    rustup install nightly
    echo # things required by `just test`
    cargo install cargo-nextest
    echo # things required by `just pre-commit`
    cargo install cargo-spellcheck --locked
    cargo spellcheck completions # shell auto-completions for cargo-spellcheck
    echo # things required by `just thorough-check`
    rustup component add llvm-tools-preview # required by cargo-llvm-cov
    cargo install \
      cargo-udeps --locked \
      cargo-llvm-cov \
      cargo-audit \
      cargo-criterion \
      cargo-upgrades
