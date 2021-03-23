# For Maintainer

## Publishing flow

Testing and publishing are automatically done in GitHub Actions.
The definitions are located under `.github` directory.

### Rust

1. Get a token from crates.io and set it into `CRATES_PASS` secrets via the settings page of this repository
    - Token can be issued in "API Access" section in [crates.io account settings page](https://crates.io/me)
1. Fix version in `Cargo.toml` (e.g. 0.1.0)
1. Git tag version with prefix `rust/` (e.g. `git tag rust/0.1.0`)
1. Push tag to master
1. CI automatically publish a new crate to crates.io after testing

### Python

1. Set the PyPi user name and password into `PYPI_USER` and `PYPI_PASS` respectively, via the settings page of this repository.
1. Fix version in `python/pyproject.toml`, `python/Cargo.toml`, `python/src/lib.rs`
    - Easily done with [pyversionup](https://github.com/tamuhey/pyversionup): e.g. `versionup 0.1.0`
1. Git tag version with prefix `python/`
1. Push tag to master
1. CI automatically publish package to PyPi after testing

