# Rust for CDM-16

This is a fork of the Rust compiler and toolchain with experimental support for targeting the [CDM-16 educational processor]. It is based on the [Neo CDM-16 LLVM backend].

You would usually want to use the hardware support crates from [`cdm-rs`] in your projects. They provide startup logic and utilities for controlling processor interrupts.

For a complete usage example, check out [`cdm-paint-rs`].

[CDM-16 educational processor]: https://github.com/cdm-processors/cdm-devkit/blob/d1b647bb8ae9db43be7757a76fa63ddb112fa450/docs/cdm16/cdm16-overview.md
[Neo CDM-16 LLVM backend]: https://github.com/ylab-nsu/cdm16-llvm-neo/
[`cdm-rs`]: https://github.com/aelsi2/cdm-rs
[`cdm-paint-rs`]: https://github.com/aelsi2/cdm-paint-rs

## Installation

[![Nightly Release](https://img.shields.io/github/v/release/ylab-nsu/cdm16-rust?include_prereleases&filter=cdm-nightly)](https://github.com/ylab-nsu/cdm16-rust/releases/cdm-nightly)

Requirements: `rustup`.

```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/ylab-nsu/cdm16-rust/cdm/cdm_install.sh | sh
```

<details>
<summary>I don't like curl | sh</summary>

Install the nightly toolchain with `rustup`.
```sh
rustup toolchain install nightly
```

Download the latest `rustc`, `rust-std` and `rust-src` packages for your platform (currently Linux x86\_64 and macOS AArch64 are supported).

```sh
# Linux
curl -OL https://github.com/ylab-nsu/cdm16-rust/releases/download/cdm-nightly/rustc-nightly-x86_64-unknown-linux-gnu.tar.gz
curl -OL https://github.com/ylab-nsu/cdm16-rust/releases/download/cdm-nightly/rust-std-nightly-x86_64-unknown-linux-gnu.tar.gz
curl -OL https://github.com/ylab-nsu/cdm16-rust/releases/download/cdm-nightly/rust-src-nightly.tar.gz
```
```sh
# macOS
curl -OL https://github.com/ylab-nsu/cdm16-rust/releases/download/cdm-nightly/rustc-nightly-aarch64-apple-darwin.tar.gz
curl -OL https://github.com/ylab-nsu/cdm16-rust/releases/download/cdm-nightly/rust-std-nightly-aarch64-apple-darwin.tar.gz
curl -OL https://github.com/ylab-nsu/cdm16-rust/releases/download/cdm-nightly/rust-src-nightly.tar.gz
```

Extract the packages.
```sh
# Linux
tar -xzf rustc-nightly-x86_64-unknown-linux-gnu.tar.gz
tar -xzf rust-std-nightly-x86_64-unknown-linux-gnu.tar.gz
tar -xzf rust-src-nightly.tar.gz
```
```sh
# macOS
tar -xzf rustc-nightly-aarch64-apple-darwin.tar.gz
tar -xzf rust-std-nightly-aarch64-apple-darwin.tar.gz
tar -xzf rust-src-nightly.tar.gz
```

Create a directory for the CDM-16 Rust toolchain, for example `~/.rust-cdm`.
```sh
mkdir ~/.rust-cdm
```

Run the `install.sh` script from each of the extracted directories with `--prefix="PATH TO TOOLCHAIN DIR"`.
```sh
# Linux
./rustc-nightly-x86_64-unknown-linux-gnu/install.sh --prefix=~/.rust-cdm
./rust-std-nightly-x86_64-unknown-linux-gnu/install.sh --prefix=~/.rust-cdm
./rust-src-nightly/install.sh --prefix=~/.rust-cdm
```
```sh
# macOS
./rustc-nightly-aarch64-apple-darwin/install.sh --prefix=~/.rust-cdm
./rust-std-nightly-aarch64-apple-darwin/install.sh --prefix=~/.rust-cdm
./rust-src-nightly/install.sh --prefix=~/.rust-cdm
```

Register the installed toolchain in `rustup`.
```sh
rustup toolchain link cdm ~/.rust-cdm
```
</details>
