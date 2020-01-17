# Publishment flow

## Rust

1. Fix version in `Cargo.toml` (e.g. 0.1.0)
1. Git tag version with prefix `rust/` (e.g. `git tag rust/0.1.0)
1. Push tag to master
1. CI automatically publish crates to crates.io after testing

## Python

1. Fix version in `python/pyproject.toml`, `python/Cargo.toml`, `python/src/lib.rs`
    - Easily done with [pyversionup](https://github.com/tamuhey/pyversionup): `versionup 0.1.0`
1. Git tag version with prefix `python/`
1. Push tag to master
1. CI automatically publish package to pypi after testing
