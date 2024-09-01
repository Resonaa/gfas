# Contributing to gfas

Thanks for your interest in contributing to gfas!

## Formatting

Use [Rustfmt](https://github.com/rust-lang/rustfmt) to format Rust code:

```sh
$ rustup component add rustfmt --toolchain nightly
$ cargo +nightly fmt
```

## Pre-commit Hooks

Use [pre-commit](https://pre-commit.com) to manage Git pre-commit hooks:

```sh
$ pip install pre-commit
$ pre-commit install
```

## Coverage

Use [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) to generate coverage results:

```sh
$ cargo install cargo-tarpaulin
$ cargo tarpaulin
```

## Releasing

Use [cargo-release](https://crates.io/crates/cargo-release) to automate release process:

```sh
$ cargo install cargo-release
$ cargo release <LEVEL>
```
