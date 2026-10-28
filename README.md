# Cargo Go [![Version][version-img]][version-url] [![CI][ci-img]][ci-url]

A Cargo subcommand to ease navigation to the `crates.io`, documentation, home,
and repository pages of a package.

## Installation

```bash
cargo install cargo-go
```

Prebuilt binaries are also available through
[`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall cargo-go
```

## Usage

```bash
cargo go crates foo        # go to foo on crates.io
cargo go docs foo          # go to foo’s documentation
cargo go home foo          # go to foo’s home page
cargo go repo foo          # go to foo’s repository
```

The destination comes before the package name. Each command opens the selected
page in your browser.

## Recipes

```bash
alias cgo='cargo go repo "$(basename "${PWD}")"'  # go to the repository of the current package
```

## Development

Build and test the project locally with:

```bash
cargo build
cargo test
cargo clippy --all-targets
```

Clippy denies missing documentation for private items as well as public ones.
Add a short doc comment to every function and other item so this check passes.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

## Releasing

To publish a release, update the version in `Cargo.toml` and run the **Release**
workflow manually from the commit to release. The workflow publishes the crate
to crates.io and attaches Linux, macOS, and Windows binaries to the matching
GitHub Release. Configure a repository Actions secret named
`CARGO_REGISTRY_TOKEN` with a crates.io publish token before running it.

[ci-img]: https://github.com/IvanUkhov/cargo-go/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/IvanUkhov/cargo-go/actions/workflows/ci.yml
[version-img]: https://img.shields.io/crates/v/cargo-go.svg
[version-url]: https://crates.io/crates/cargo-go
