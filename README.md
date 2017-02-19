# Cargo Go [![Version][version-img]][version-url] [![Status][status1-img]][status1-url] [![Status][status2-img]][status2-url]

A Cargo subcommand to ease navigation to the `crates.io`, documentation, home,
and repository pages of a package.

## Installation

```bash
cargo install cargo-go
```

## Usage

```bash
cargo go foo        # go to foo on crates.io
cargo go foo docs   # go to foo’s documentation
cargo go foo home   # go to foo’s home page
cargo go foo repo   # go to foo’s repository
```

## Recipes

```bash
alias cgo='cargo go $(basename ${PWD}) repo'  # go to the repository of the current package
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[status1-img]: https://travis-ci.org/IvanUkhov/cargo-go.svg?branch=master
[status1-url]: https://travis-ci.org/IvanUkhov/cargo-go
[status2-img]: https://ci.appveyor.com/api/projects/status/j33t6axedga9c0t7?svg=true
[status2-url]: https://ci.appveyor.com/project/IvanUkhov/cargo-go
[version-img]: https://img.shields.io/crates/v/cargo-go.svg
[version-url]: https://crates.io/crates/cargo-go
