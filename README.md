<div align="center">

# gfas

**G**itHub **F**ollowing **A**uto **S**ynchronization

[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/jwcub/gfas/build.yml?style=flat-square)](https://github.com/jwcub/gfas/actions)
[![License](https://img.shields.io/github/license/jwcub/gfas?style=flat-square&color=orange)](https://github.com/jwcub/gfas/blob/main/LICENSE)
[![GitHub repo size](https://img.shields.io/github/repo-size/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas)
[![GitHub Repo stars](https://img.shields.io/github/stars/jwcub/gfas?style=flat-square&color=yellow)](https://github.com/jwcub/gfas/stargazers)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas/commits/main/)
[![GitHub contributors](https://img.shields.io/github/contributors/jwcub/gfas?style=flat-square)](https://github.com/jwcub/gfas/graphs/contributors)

</div>

## Overview

`gfas` synchronizes your GitHub followings to your followers. It runs on CLI and can be
scheduled automatically with GitHub Actions.

## Quick Start

- Fork this repository.
- Create a personal access token in GitHub settings (only `user:follow` is required).
- Set the token as a secret named `TOKEN` of the repository.
- (Optional) Modify the scheduled time in `.github/workflow/sync.yml`. By default, it runs at 0 minutes past the hour every 6 hours.
- (Optional) Maybe you want to trigger the `Sync` workflow manually to see the effect.
- You are ready to follow back  automatically!

## CLI Usage

```plaintext
$ cargo run --release -- --help
Sync GitHub followings to followers

Usage: gfas.exe [OPTIONS] --user <USER> --token <TOKEN>

Options:
  -u, --user <USER>    Current user
  -t, --token <TOKEN>  Access token
  -d, --dry            Dry run
  -h, --help           Print help
```

## License

This project is licensed under the [Unlicense](https://github.com/jwcub/gfas/blob/main/LICENSE).
