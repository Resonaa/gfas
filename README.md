<div align="center">

# gfas

**G**itHub **F**ollowing **A**uto **S**ynchronization

[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/jwcub/gfas/ci.yml?style=flat-square)](https://github.com/jwcub/gfas/actions)
[![Crates.io](https://img.shields.io/crates/v/gfas-cli?style=flat-square)](https://crates.io/crates/gfas-cli)
[![Downloads](https://img.shields.io/crates/d/gfas-cli?style=flat-square)](https://crates.io/crates/gfas-cli)
[![License](https://img.shields.io/github/license/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas/blob/main/LICENSE)
[![GitHub repo size](https://img.shields.io/github/repo-size/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas)
[![GitHub Repo stars](https://img.shields.io/github/stars/jwcub/gfas?style=flat-square&color=yellow)](https://github.com/jwcub/gfas/stargazers)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas/commits/main/)
[![GitHub contributors](https://img.shields.io/github/contributors/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas/graphs/contributors)

</div>

## Overview

`gfas` synchronizes your GitHub followings to your followers. It runs on CLI and can be
scheduled automatically with [GitHub Actions](https://docs.github.com/actions).

## Usage

### GitHub Actions

- Fork this repository.
- Create a [personal access token](https://docs.github.com/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens) with the `user:follow` permission.
- Create an [secret](https://docs.github.com/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions) named `TOKEN` in the repository containing the token value.
- You are ready to follow back automatically!
- (Optional) Modify the [scheduled time](https://docs.github.com/actions/writing-workflows/choosing-when-your-workflow-runs/events-that-trigger-workflows#schedule) in `.github/workflow/sync.yml`. By default, it runs at 0 minutes past the hour every 6 hours.
- (Optional) Maybe you want to trigger the `Sync` workflow manually to see the effect.

### CLI

```sh
$ cargo install gfas-cli
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

### API

The [gfas-api](https://crates.io/crates/gfas-api) crate exports some [GitHub API bindings](https://docs.rs/gfas-api) which can be used to build your application.

```sh
$ cargo add gfas-api
```

```rust
use gfas_api::GitHub;

let github = GitHub::with_token("<TOKEN>")?;

github.follow("<USER-TO-FOLLOW>").await?;
```

## Development

```sh
$ git clone https://github.com/jwcub/gfas.git
$ cd gfas
$ cargo run --release -- --help
```

## Contributing

Before pushing your commits, be sure to run through all the checks:

```sh
$ cargo clippy
$ cargo fmt
$ cargo build
$ cargo doc --no-deps
```

## License

This project is licensed under the [Unlicense](https://github.com/jwcub/gfas/blob/main/LICENSE).
