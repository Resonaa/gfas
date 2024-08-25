# gfas

gfas (**G**itHub **F**ollowing **A**uto **S**ynchronization) manipulates your
GitHub following list to match your followers, enabling you to follow back
and unfollow back automatically. It runs on CLI and can be scheduled with
[GitHub Actions](https://docs.github.com/actions).

[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/jwcub/gfas/ci.yml?style=flat-square)](https://github.com/jwcub/gfas/actions)
[![Coverage](https://img.shields.io/codecov/c/github/jwcub/gfas?token=W3H6GBVQZW&style=flat-square
)](https://app.codecov.io/github/jwcub/gfas)
[![Crates.io](https://img.shields.io/crates/v/gfas-cli?style=flat-square)](https://crates.io/crates/gfas-cli)
[![Downloads](https://img.shields.io/crates/d/gfas-cli?style=flat-square)](https://crates.io/crates/gfas-cli)
[![License](https://img.shields.io/github/license/jwcub/gfas?style=flat-square)](LICENSE)

## Usage

### GitHub Actions

- Copy-paste the
[Sync](.github/workflows/sync.yml)
workflow to any of your repositories. You can just fork this repository for convenience.
- Create a
[personal access token](https://docs.github.com/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
with the `user:follow` permission.
- Create an
[secret](https://docs.github.com/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions)
named `TOKEN` in the repository containing the token value.
- Customize your workflow, e.g. modify the [scheduled time](https://docs.github.com/actions/writing-workflows/choosing-when-your-workflow-runs/events-that-trigger-workflows#schedule) (defaults to every 6 hours).
- Maybe you want to trigger it manually to see the effect.

### CLI

```sh
$ cargo install gfas-cli
$ gfas sync -u <your-username> -t <your-token>
```

Run `gfas --help` for all commands and options.

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

let github = GitHub::with_token("<TOKEN>")?;

github.follow("<USER-TO-FOLLOW>").await?;
```

## Development

```sh
$ git clone https://github.com/jwcub/gfas.git
$ cd gfas
$ cargo run -- --help
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the [Unlicense](LICENSE).
