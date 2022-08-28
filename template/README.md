# {{project-name}}

[![Crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![Docs.rs](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}})
[![CI](https://github.com/{{gh-username}}/{{project-name}}/workflows/CI/badge.svg)](https://github.com/{{gh-username}}/{{project-name}}/actions)
[![Coverage Status](https://coveralls.io/repos/github/{{gh-username}}/{{project-name}}/badge.svg?branch=main)](https://coveralls.io/github/{{gh-username}}/{{project-name}}?branch=main)

# Development

Only requires `just` to bootstrap all tools and configuration.
However, using `cargo-binstall` is recommended and utilised heavily in `just init` command.
```bash
cargo install cargo-binstall
cargo binstall just
just init # setup repo, install hooks and all required tools
```
{% if crate_type == "bin" %}
To run:
```bash
just run
```
{% endif %}
To test:
```bash
just test
```

Before committing work:
```bash
just pre-commit
```

Too see all available commands:
```bash
just list
```

## TODO
- [ ] Next thing to do in the project.

## Similar projects

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[LICENSE-APACHE]: ./LICENSE-APACHE
[LICENSE-MIT]: ./LICENSE-MIT
