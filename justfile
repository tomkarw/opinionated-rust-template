#!/usr/bin/env just --justfile
set dotenv-load := true

library-name := "opinonated-library"
binary-name := "opinonated-binary"

list:
    @just --list

help:
    @just list

generate-opinonated-library:
    cargo generate --path . --lib --name {{library-name}} -d project-description="Example library project using opinionated-rust-template" -d nightly=true -d protect-main-branch=true -d gh-username=tomkarw

generate-opinonated-binary:
    cargo generate --path . --bin --name {{binary-name}} -d project-description="Example binary project using opinionated-rust-template" -d nightly=true -d protect-main-branch=true -d gh-username=tomkarw

generate:
    rm -rf {{library-name}}
    rm -rf {{binary-name}}
    @just generate-opinonated-library
    @just generate-opinonated-binary

pre-commit:
    @just generate
    cd {{library-name}} && just check -- -D warnings
    cd {{binary-name}} && just check -- -D warnings
