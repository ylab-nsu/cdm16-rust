# Rust for CDM-16 

[![Nightly Release](https://img.shields.io/github/v/release/ylab-nsu/cdm16-rust?include_prereleases&filter=cdm-nightly)](https://github.com/ylab-nsu/cdm16-rust/releases/cdm-nightly)

This is a fork of the Rust compiler and toolchain with experimental [CDM-16 educational processor] target support. It is based on the [Neo CDM-16 LLVM backend].

[`cdm-rs`] provides startup logic and various utilities for working with the processor.

For a complete usage example, check out [`cdm-paint-rs`].

[CDM-16 educational processor]: https://github.com/cdm-processors/cdm-devkit/blob/d1b647bb8ae9db43be7757a76fa63ddb112fa450/docs/cdm16/cdm16-overview.md
[Neo CDM-16 LLVM backend]: https://github.com/ylab-nsu/cdm16-llvm-neo/
[`cdm-rs`]: https://github.com/aelsi2/cdm-rs
[`cdm-paint-rs`]: https://github.com/aelsi2/cdm-paint-rs

## Installation
Use the easy installer:
```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/ylab-nsu/cdm16-rust/cdm/cdm-install.sh | sh
```

Requirements: 
- Linux x86\_64 or macOS AArch64
- `bash`
- `rustup`

<details>
<summary><b>Manual installation</b></summary>
<br>

Download the latest `rust-nightly` package for your platform.

```sh
# Linux
curl --proto '=https' --tlsv1.2 -fOL https://github.com/ylab-nsu/cdm16-rust/releases/download/cdm-nightly/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
```
```sh
# macOS
curl --proto '=https' --tlsv1.2 -fOL https://github.com/ylab-nsu/cdm16-rust/releases/download/cdm-nightly/rust-nightly-aarch64-apple-darwin.tar.gz
```

Extract the package.
```sh
# Linux
tar -xzf rust-nightly-x86_64-unknown-linux-gnu.tar.gz
```
```sh
# macOS
tar -xzf rust-nightly-aarch64-apple-darwin.tar.gz
```

Create a directory for the CDM-16 Rust toolchain, for example `~/.rust-cdm`.
```sh
mkdir ~/.rust-cdm
```

Run the `install.sh` script the extracted directory with `--prefix="PATH TO TOOLCHAIN DIR"`.
```sh
# Linux
./rust-nightly-x86_64-unknown-linux-gnu/install.sh --prefix=~/.rust-cdm
```
```sh
# macOS
./rust-nightly-aarch64-apple-darwin/install.sh --prefix=~/.rust-cdm
```

Register the installed toolchain in `rustup`.
```sh
rustup toolchain link cdm ~/.rust-cdm
```

If you want to use `cargo objcopy` to automatically convert ELF object files to Logisim image format,
install [`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils/).
```sh
rustup run cdm cargo install cargo-binutils
```
</details>
