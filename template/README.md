# {{project-name}}
[![crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{crate_name}})
[![docs.rs](https://img.shields.io/docsrs/{{project-name}})](https://docs.rs/{{crate_name}})
[![CI](https://github.com/{{gh-username}}/{{project-name}}/workflows/CI/badge.svg)](https://github.com/{{gh-username}}/{{project-name}}/actions?query=workflow%3ACI)
[![coverage status](https://coveralls.io/repos/github/{{gh-username}}/{{project-name}}/badge.svg?branch=main)](https://coveralls.io/github/{{gh-username}}/{{project-name}}?branch=main)
[![APACHE licensed](https://shields.io/github/license/{{gh-username/{{project-name.svg)](https://github.com/{{gh-username}}/{{project-name}}/blob/main/LICENSE-APACHE)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/{{gh-username}}/{{project-name}}/blob/main/LICENSE-MIT)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

# Development

Only requires `just` to bootstrap all tools and configuration.
```bash
cargo install just
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

To see all available commands:
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
