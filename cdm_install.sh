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
    OS="$(uname -sm)"
    case "$OS" in
        "Darwin arm64")
            echo aarch64-apple-darwin
            ;;
        "Linux x86_64")
            echo x86_64-unknown-linux-gnu
            ;;
        *)
            echo "Sorry, your operating system ($OS) is not supported yet." >&2
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
            echo $1
            ;;
    esac
}

get_default_install_dir() {
    if [ -z "$HOME" ]; then
        echo "Can't determine the default install directory because \$HOME is not set." >&2
        echo "Please, re-log into the system or specify the install directory explicitly" >&2
        return 1
    fi
    echo "$HOME/.rust-cdm"
}

check_tools() {
    RESULT=0
    for tool in "$@"; do
        if ! command -v "$tool" >/dev/null; then
            echo "Missing tool: $tool" >&2
            RESULT=1
        fi
    done
    if [ "$RESULT" -ne 0 ]; then
        echo "Please install these with your package manager." >&2
    fi
    return $RESULT
}

check_install_dir() {
    if [ -f "$1" ]; then
        echo "Install directory is a file: $1" >&2
        echo "Please, specify a different install directory or remove the file." >&2
        return 1
    fi
    if [ -d "$1" ] && [ -n "$(ls -A "$1")" ]; then
        echo "Install directory is not empty: $1" >&2
        echo "Please, specify a different install directory or remove the contents of the directory." >&2
        return 1
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
    for toolchain in $(rustup toolchain list | awk '{print $1}'); do
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
        resp=$(expand_tilde "$resp")
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


INSTALL_DIR=
VERSION=
TRIPLE=
TOOLCHAIN_NAME=
NO_ASK=

while getopts "d:v:t:sh" opt; do
    case $opt in
        d) INSTALL_DIR="$OPTARG" ;;
        v) VERSION="$OPTARG" ;;
        t) TOOLCHAIN_NAME="$OPTARG" ;;
        s) NO_ASK=1 ;;
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

TRIPLE=$(get_triple)
if [ -z "$INSTALL_DIR" ]; then
    INSTALL_DIR=$(get_default_install_dir)
fi
if [ -z "$VERSION" ]; then
    VERSION=cdm-nightly
fi
if [ -z "$TOOLCHAIN_NAME" ]; then
    TOOLCHAIN_NAME=cdm
fi
if [ -z "$NO_ASK" ]; then
    NO_ASK=0
fi

check_tools rustup curl
check_install_dir "$INSTALL_DIR"
check_rustup_toolchain "$TOOLCHAIN_NAME"

if [ "$NO_ASK" -eq 0 ]; then
    prompt_input VERSION true "Enter release name ($VERSION):"
    prompt_input INSTALL_DIR check_install_dir "Enter install directory ($INSTALL_DIR):"
    prompt_input TOOLCHAIN_NAME check_rustup_toolchain "Enter toolchain name ($TOOLCHAIN_NAME):"
    echo
fi

echo "Host platform:            $TRIPLE"
echo "Toolchain version:        $VERSION"
echo "Install directory:        $INSTALL_DIR"
echo "Rustup toolchain name:    $TOOLCHAIN_NAME"
echo
if [ "$NO_ASK" -eq 0 ] && ! confirm_install "Proceed with installation? (Y/n)"; then
    echo "Installation canceled."
    exit 0
fi

DOWNLOAD_DIR=$(mktemp -d /tmp/rust-cdm-download.XXXXXX)

SUCCESS=0
cleanup() {
    rm -rf "$DOWNLOAD_DIR"
    if [ "$SUCCESS" -eq 0 ]; then
        echo "Installation failed. Removing directory $INSTALL_DIR" >&2
        rm -rf "$INSTALL_DIR"
    fi
}
trap cleanup EXIT

DOWNLOAD_URL="https://github.com/ylab-nsu/cdm16-rust/releases/download"
echo "Downloading rustc..."
curl --proto "=https" -#fLo "$DOWNLOAD_DIR/rustc.tar.gz" "$DOWNLOAD_URL/$VERSION/rustc-nightly-$TRIPLE.tar.gz"
echo "Downloading rust-std..."
curl --proto "=https" -#fLo "$DOWNLOAD_DIR/rust-std.tar.gz" "$DOWNLOAD_URL/$VERSION/rust-std-nightly-$TRIPLE.tar.gz"
echo "Downloading rust-src..."
curl --proto "=https" -#fLo "$DOWNLOAD_DIR/rust-src.tar.gz" "$DOWNLOAD_URL/$VERSION/rust-src-nightly.tar.gz"

mkdir -p "$INSTALL_DIR"

COMPONENTS="rustc rust-std rust-src"
for comp in $COMPONENTS; do
    mkdir -p "$DOWNLOAD_DIR/$comp"
    echo "Extracting $comp..."
    tar -xzf "$DOWNLOAD_DIR/$comp.tar.gz" -C "$DOWNLOAD_DIR/$comp" --strip-components=1
    echo "Installing $comp..."
    "$DOWNLOAD_DIR/$comp/install.sh" --prefix="$INSTALL_DIR"
done

echo "Linking CDM-16 toolchain..."
rustup toolchain link "$TOOLCHAIN_NAME" "$INSTALL_DIR"

echo "Installing nightly toolchain..."
rustup toolchain install nightly

echo "Installation completed successfully."
SUCCESS=1
