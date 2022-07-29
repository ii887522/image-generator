# image-generator
[![Semantic Versioning 2.0.0](https://img.shields.io/badge/semver-2.0.0-standard.svg)](https://semver.org/)
[![Linux](https://svgshare.com/i/Zhy.svg)](https://svgshare.com/i/Zhy.svg)
[![Windows](https://svgshare.com/i/ZhY.svg)](https://svgshare.com/i/ZhY.svg)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

A CLI tool used to generate many sample images with different sizes and colors for testing purposes.

## Table of contents
- [Prerequisites](https://github.com/ii887522/image-generator#prerequisites)
- [Format the project](https://github.com/ii887522/image-generator#format-the-project)
- [Automatically format the project on change](https://github.com/ii887522/image-generator#automatically-format-the-project-on-change)
- [Lint the project](https://github.com/ii887522/image-generator#lint-the-project)
- [Automatically lint the project on change](https://github.com/ii887522/image-generator#automatically-lint-the-project-on-change)
- [Build the vcpkg dependencies in the project](https://github.com/ii887522/image-generator#build-the-vcpkg-dependencies-in-the-project)
- [Build the project](https://github.com/ii887522/image-generator#build-the-project)
- [Automatically build the project on change](https://github.com/ii887522/image-generator#automatically-build-the-project-on-change)
- [Test the project](https://github.com/ii887522/image-generator#test-the-project)
- [Automatically test the project on change](https://github.com/ii887522/image-generator#automatically-test-the-project-on-change)
- [Run the project](https://github.com/ii887522/image-generator#run-the-project)

## Prerequisites
- Windows 11 or Linux
- [Visual Studio Code](https://code.visualstudio.com/) with plugins:
  - Better TOML
  - CodeLLDB
  - EditorConfig for VS Code
  - GLSL Lint
  - Markdown All in One
  - rust-analyzer
  - Shader languages support for VS Code
  - YAML
- [Rust 1.61.0](https://www.rust-lang.org/) and later
- [rustfmt 1.4.38](https://github.com/rust-lang/rustfmt) and later
- [clippy 0.1.60](https://github.com/rust-lang/rust-clippy) and later
- [cargo-watch 8.1.1](https://github.com/watchexec/cargo-watch) and later
- [cargo-vcpkg 0.1.6](https://crates.io/crates/cargo-vcpkg) and later

## Format the project
```sh
cargo fmt
```

## Automatically format the project on change
```sh
cargo watch -x fmt
```

## Lint the project
```sh
cargo clippy --all-features
```

## Automatically lint the project on change
```sh
cargo watch -x "clippy --all-features"
```

## Build vcpkg dependencies in the project
```sh
cargo vcpkg build
```

## Build the project
```sh
cargo build
```

## Automatically build the project on change
```sh
cargo watch -x build
```

## Test the project
```sh
cargo test
```

## Automatically test the project on change
```sh
cargo watch -x test
```

## Run the project
```sh
cargo run
```
