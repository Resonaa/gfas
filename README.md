<div align="center">

# gfas

**G**itHub **F**ollowing **A**uto **S**ynchronization

[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/jwcub/gfas/build.yml?style=flat-square)](https://github.com/jwcub/gfas/actions)
[![Crates.io](https://img.shields.io/crates/v/gfas?style=flat-square)](https://crates.io/crates/gfas)
[![Downloads](https://img.shields.io/crates/d/gfas?style=flat-square)](https://crates.io/crates/gfas)
[![License](https://img.shields.io/github/license/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas/blob/main/LICENSE)
[![GitHub repo size](https://img.shields.io/github/repo-size/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas)
[![GitHub Repo stars](https://img.shields.io/github/stars/jwcub/gfas?style=flat-square&color=yellow)](https://github.com/jwcub/gfas/stargazers)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas/commits/main/)
[![GitHub contributors](https://img.shields.io/github/contributors/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas/graphs/contributors)

</div>

## Overview

`gfas` synchronizes your GitHub followings to your followers. It runs on CLI and can be
scheduled automatically with GitHub Actions.

## Usage

### GitHub Actions

- Fork this repository.
- Create a personal access token in GitHub settings (only `user:follow` is required).
- Set the token as a secret named `TOKEN` of the repository.
- (Optional) Modify the scheduled time in `.github/workflow/sync.yml`. By default, it runs at 0 minutes past the hour every 6 hours.
- (Optional) Maybe you want to trigger the `Sync` workflow manually to see the effect.
- You are ready to follow back  automatically!

### CLI

#### Installing With Cargo

```sh
$ cargo install gfas
$ gfas --help
Sync GitHub followings to followers

Usage: gfas.exe [OPTIONS] --user <USER> --token <TOKEN>

Options:
  -u, --user <USER>    Current user
  -t, --token <TOKEN>  Access token
  -v, --verbose...     Increase logging verbosity
  -q, --quiet...       Decrease logging verbosity
  -h, --help           Print help
  -V, --version        Print version
```

#### Building From Source

```sh
$ git clone https://github.com/jwcub/gfas.git
$ cd gfas
$ cargo build --release
$ ./target/release/gfas --help
```

### API

This crate also exports some GitHub API bindings as a library which can be used to build your application.

```sh
$ cargo add gfas
```

```rust
use gfas::GitHub;

let github = GitHub::with_token("<TOKEN>")?;

github.follow("<USER-TO-FOLLOW>").await?;
```

Refer to the [Documentation](https://docs.rs/gfas/latest/gfas/) for more information.

## Development

Before pushing your commits, be sure to run through all the checks:

```sh
$ cargo clippy
$ cargo fmt
$ cargo build
$ cargo doc --no-deps --lib
```

## License

This project is licensed under the [Unlicense](https://github.com/jwcub/gfas/blob/main/LICENSE).
