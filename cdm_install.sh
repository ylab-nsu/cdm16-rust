#!/bin/sh

set -e

print_help() {
    echo "CDM-16 Rust toolchain install script"
    echo "Usage: $0 [-d DIRECTORY] [-v RELEASE] [-t NAME] [-s] [-h]"
    echo " -d DIRECTORY : The directory to install the Rust toolchain to. Default: ~/.rust-cdm"
    echo " -v VERSION   : The name of the release to download. Default: cdm-nightly"
    echo " -t NAME      : The name of the toolchain to use in rustup. Default: cdm"
    echo " -s           : Script mode. Don't ask interactive questions."
    echo " -h           : Display this help message."
}

get_triple() {
    os="$(uname -sm)"
    case "$os" in
        "Darwin arm64")
            echo aarch64-apple-darwin
            ;;
        "Linux x86_64")
            echo x86_64-unknown-linux-gnu
            ;;
        *)
            echo "Sorry, your operating system ($os) is not supported yet." >&2
            return 1
            ;;
    esac
}

expand_tilde() {
    case "$1" in
        "~/"* | "~")
            if [ -z "$HOME" ]; then
                echo "Can't expand '~' because \$HOME is not set." >&2
                echo "Please, re-log into the system or specify the full path explicitly" >&2
                return 1
            fi
            home_path=$1; home_path=${home_path#'~/'}; home_path=${home_path#'~'}
            echo "$HOME/$home_path"
            ;;
        *)
            echo "$1"
            ;;
    esac
}

get_default_install_dir() {
    if [ -z "$HOME" ]; then
        if [ "$1" -eq 0 ]; then
            echo "./.rust-cdm"
            return 0
        else
            echo "Can't determine the default install directory because \$HOME is not set." >&2
            echo "Please, re-log into the system or specify the directory with -d" >&2
            return 1
        fi
    fi
    echo "$HOME/.rust-cdm"
}

check_tools() {
    result=0
    for tool in "$@"; do
        if ! command -v "$tool" >/dev/null; then
            echo "Missing tool: $tool" >&2
            result=1
        fi
    done
    if [ "$result" -ne 0 ]; then
        echo "Please install these with your package manager." >&2
    fi
    return $result
}

check_install_dir() {
    if [ -f "$1" ]; then
        echo "Install directory is a file: $1" >&2
        echo "Please, specify a different install directory or remove the file." >&2
        return 1
    fi
    if [ -d "$1" ]; then
        ls_output=$(ls -A "$1")
        if [ -n "$ls_output" ]; then
            echo "Install directory is not empty: $1" >&2
            echo "Please, specify a different install directory or remove the contents of the directory." >&2
            return 1
        fi
    fi
}

check_rustup_toolchain() {
    case "$1" in
      . | ..)
          echo "Toolchain name must not be '.' or '..'" >&2
          echo "Please specify a different toolchain name" >&2
          return 1
          ;;
      */* | *\\*)
          echo "Toolchain name must not contain '/' or '\\': $1" >&2
          echo "Please specify a different toolchain name" >&2
          return 1
          ;;
      stable* | beta* | nightly* | none)
          echo "Toolchain name is reserved in Rustup: $1" >&2
          echo "Please specify a different toolchain name" >&2
          return 1
          ;;
    esac
    toolchains=$(rustup toolchain list)
    for toolchain in $(echo "$toolchains" | awk '{print $1}'); do
        if [ "$toolchain" = "$1" ]; then
            echo "Rustup toolchain already exists: $toolchain" >&2
            echo "Please specify a different toolchain name" >&2
            return 1
        fi
    done
}

prompt_input() {
    while true; do
        printf "%s " "$3"
        read resp </dev/tty
        resp=$(expand_tilde "$resp") || continue
        if [ -z "$resp" ]; then
            eval "resp=\"\$$1\""
        fi
        if $2 $resp; then
            eval "$1=\"\$resp\""
            return 0
        fi
    done
}

confirm_install() {
    while true; do
        printf "%s " "$1"
        read resp </dev/tty
        case "$resp" in
            [Yy]* | '')
                return 0
                ;;
            [Nn]*)
                return 1
                ;;
            *)
                echo "Please answer yes or no."
                ;;
        esac
    done
}


install_dir=
version=
triple=
toolchain_name=
no_ask=

while getopts "d:v:t:sh" opt; do
    case $opt in
        d) install_dir="$OPTARG" ;;
        v) version="$OPTARG" ;;
        t) toolchain_name="$OPTARG" ;;
        s) no_ask=1 ;;
        h)
            print_help
            exit 0
            ;;
        \?)
            print_help >&2
            exit 1
            ;;
    esac
done

triple=$(get_triple)
check_tools rustup curl

if [ -z "$no_ask" ]; then
    no_ask=0
fi
if [ -z "$install_dir" ]; then
    install_dir=$(get_default_install_dir $no_ask)
fi
if [ -z "$version" ]; then
    version=cdm-nightly
fi
if [ -z "$toolchain_name" ]; then
    toolchain_name=cdm
fi

if [ "$no_ask" -eq 0 ]; then
    prompt_input version true "Enter release name ($version):"
    prompt_input install_dir check_install_dir "Enter install directory ($install_dir):"
    prompt_input toolchain_name check_rustup_toolchain "Enter toolchain name ($toolchain_name):"
    echo
fi

check_install_dir "$install_dir"
check_rustup_toolchain "$toolchain_name"

echo "Host platform:            $triple"
echo "Toolchain version:        $version"
echo "Install directory:        $install_dir"
echo "Rustup toolchain name:    $toolchain_name"
echo
if [ "$no_ask" -eq 0 ] && ! confirm_install "Proceed with installation? (Y/n)"; then
    echo "Installation canceled."
    exit 0
fi

download_dir=$(mktemp -d "${TMPDIR:-/tmp}/rust-cdm-download.XXXXXX")

install_success=0
cleanup() {
    rm -rf "$download_dir"
    if [ "$install_success" -eq 0 ]; then
        echo "Installation failed. Removing directory $install_dir" >&2
        rm -rf "$install_dir"
    fi
}
trap cleanup EXIT

download_url="https://github.com/ylab-nsu/cdm16-rust/releases/download"
echo "Downloading rustc..."
curl --proto "=https" -#fLo "$download_dir/rustc.tar.gz" "$download_url/$version/rustc-nightly-$triple.tar.gz"
echo "Downloading rust-std..."
curl --proto "=https" -#fLo "$download_dir/rust-std.tar.gz" "$download_url/$version/rust-std-nightly-$triple.tar.gz"
echo "Downloading rust-src..."
curl --proto "=https" -#fLo "$download_dir/rust-src.tar.gz" "$download_url/$version/rust-src-nightly.tar.gz"

mkdir -p "$install_dir"

components="rustc rust-std rust-src"
for comp in $components; do
    mkdir -p "$download_dir/$comp"
    echo "Extracting $comp..."
    tar -xzf "$download_dir/$comp.tar.gz" -C "$download_dir/$comp" --strip-components=1
    echo "Installing $comp..."
    "$download_dir/$comp/install.sh" --prefix="$install_dir"
done

echo "Linking CDM-16 toolchain..."
rustup toolchain link "$toolchain_name" "$install_dir"

echo "Installing nightly toolchain..."
rustup toolchain install nightly

echo "Installation completed successfully."
install_success=1
