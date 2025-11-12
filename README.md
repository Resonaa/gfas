# gfas

gfas (**G**itHub **F**ollowing **A**uto **S**ynchronization) manipulates your
GitHub following list to match your followers, enabling you to follow back
and unfollow back automatically. It runs on CLI and can be scheduled with
[GitHub Actions](https://docs.github.com/actions).

[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/Resonaa/gfas/ci.yml?style=flat-square)](https://github.com/Resonaa/gfas/actions)
[![Coverage](https://img.shields.io/codecov/c/github/Resonaa/gfas?token=W3H6GBVQZW&style=flat-square)](https://app.codecov.io/github/Resonaa/gfas)
[![Crates.io](https://img.shields.io/crates/v/gfas-cli?style=flat-square)](https://crates.io/crates/gfas-cli)
[![Downloads](https://img.shields.io/crates/d/gfas-cli?style=flat-square)](https://crates.io/crates/gfas-cli)
[![License](https://img.shields.io/github/license/Resonaa/gfas?style=flat-square)](LICENSE)

## Usage

### GitHub Actions

- Copy-paste the
  [Sync](.github/workflows/sync.yml)
  workflow to any of your repositories. You can just fork this repository for convenience.
- Create a
  [personal access token](https://github.com/settings/personal-access-tokens)
  with both **Read and write** access for permission `Block another user` and `Followers`.
  	- Block permission is necessary because one can still follow even after blocking us,
   		in which case we are not able to follow back. We just do a quick block-and-unblock to
     	get rid of those zombie follows.
- Create an
  [secret](https://docs.github.com/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions)
  named `TOKEN` in the repository containing the token value.
- Customize your workflow, e.g. modify the [scheduled time](https://docs.github.com/actions/writing-workflows/choosing-when-your-workflow-runs/events-that-trigger-workflows#schedule) (defaults to daily at midnight).
- Maybe you want to trigger it manually to see the effect.

### CLI

**[Archives of precompiled binaries for gfas are available for Windows,
macOS and Linux.](https://github.com/Resonaa/gfas/releases)** Linux and
Windows binaries are static executables. Users of platforms not explicitly
mentioned below are advised to download one of these archives.

If you're a **Rust programmer**, gfas can be installed with `cargo`.
Note that the minimum supported version of Rust for gfas is **1.85.0**,
although gfas may work with older versions.

```
$ cargo install gfas
```

Alternatively, one can use [`cargo
binstall`](https://github.com/cargo-bins/cargo-binstall) to install a gfas
binary directly from GitHub:

```
$ cargo binstall gfas
```

After installation, run `gfas --help` for all commands and options.

### API

The
[gfas-api](https://crates.io/crates/gfas-api)
crate exports some
[GitHub API bindings](https://docs.rs/gfas-api)
which can be used to build your application.

```sh
$ cargo add gfas-api
```

```rust
use gfas_api::GitHub;

let github = GitHub::new(String::from("<TOKEN>"))?;
let followers = github.list_followers("<USERNAME>").await?;
```

## Building

gfas is written in Rust, so you'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.
gfas compiles with Rust 1.85.0 (stable) or newer. In general, gfas tracks
the latest stable release of the Rust compiler.

To build gfas:

```sh
$ git clone https://github.com/Resonaa/gfas
$ cd gfas
$ cargo build --release
$ ./target/release/gfas --help
```

### Running tests

gfas is relatively well-tested, including both unit tests and integration
tests. To run the full test suite, use:

```
$ cargo test
```

from the repository root.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the [Unlicense](LICENSE).
