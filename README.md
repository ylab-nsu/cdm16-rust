# Rust for CDM-16 

[![Nightly Release](https://img.shields.io/github/v/release/ylab-nsu/cdm16-rust?include_prereleases&filter=cdm-nightly)](https://github.com/ylab-nsu/cdm16-rust/releases/cdm-nightly)

This is a fork of the Rust compiler and toolchain with experimental [CDM-16 educational processor] target support. It is based on the [Neo CDM-16 LLVM backend].
The compiler supports a `no_std` environment, with `core` and `alloc`.

There is an out-of-tree set of utility crates called [`cdm-rs`] that provides standard startup logic, register access functions, a minimal UART driver, etc.

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

The same command can be used to update the toolchain. Make sure that you use the same destination path when updating.

The toolchain can be uninstalled using the `lib/uninstall.sh` script placed in the install directory. Default location is `~/.rust-cdm/lib/uninstall.sh`.

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

## Usage
After completing the installation process, the toolchain will be available in `rustup` under the name you chose when you were prompted by the installer (`cdm` by default).

To use the toolchain in your project, add a local toolchain override. Run the following command in your project directory (assuming `cdm` is the name of the toolchain):
```bash
rustup override set cdm
```

Logisim memory images can be created using `cargo-objcopy` like this (`output.img` is the destination file name here):
```bash
cargo objcopy --release -- -O logisim output.img
```

Note that you don't need to run `cargo build` before running `cargo objcopy`. It will do so automatically.
