#!/bin/sh

set -e

print_help() {
    echo "CDM-16 Rust toolchain install script"
    echo "Usage: $0 [-d DIRECTORY] [-t NAME] [-v RELEASE] [-u] [-s] [-h]"
    echo " -d DIRECTORY : The directory to install the Rust toolchain to. Default: ~/.rust-cdm"
    echo " -t NAME      : The name of the toolchain to use in rustup. Default: cdm"
    echo " -v VERSION   : The name of the release to download. Default: cdm-nightly"
    echo " -u           : Update the toolchain if it already exists."
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

check_existing() {
    if ! [ -f "$1/.cdm-rust" ]; then
        return 1
    fi
    if ! [ -f "$1/cdm-uninstall.sh" ] || ! [ -x "$1/cdm-uninstall.sh" ]; then
        return 1
    fi
    . "$1/.cdm-rust"
    if [ $# -ge 2 ]; then
        eval "$2=\"\$_version\""
    fi
    if [ $# -ge 3 ]; then
        eval "$3=\"\$_toolchain_name\""
    fi
}

check_install_dir() {
    if [ -f "$1" ]; then
        echo "Install directory is a file: $1" >&2
        echo "Please, specify a different install directory or remove the file." >&2
        return 1
    fi
    if ! [ -d "$1" ]; then
        eval "$2=0"
        return 0
    fi
    if check_existing "$1" _version _toolchain_name; then
        eval "update_val=\"\$$2\""
        if [ -z "$update_val" ]; then
            echo "Directory $1 already contains toolchain version $_version"
            if ! prompt_confirm "Would you like to update the existing toolchain? (Y/n)"; then
                return 1
            fi
            update_val=1
            eval "$2=\"\$update_val\""
        fi
        if [ "$update_val" -ne 0 ]; then
            eval "toolchain_name_val=\"\$$3\""
            if [ -z "$toolchain_name_val" ]; then
                eval "$3=\"\$_toolchain_name\""
            fi
            return 0
        fi
    fi
    ls_output=$(ls -A "$1")
    if [ -n "$ls_output" ]; then
        echo "Install directory is not empty: $1" >&2
        echo "Please, specify a different install directory or remove the contents of the directory." >&2
        return 1
    fi
    eval "$2=0"
    return 0
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
    if [ "$1" = "$2" ]; then
        return 0
    fi
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
    prompt_msg="$1"
    prompt_out="$2"
    prompt_cmd="$3"
    shift 3
    while true; do
        printf "%s " "$prompt_msg"
        read resp </dev/tty
        resp=$(expand_tilde "$resp") || continue
        if [ -z "$resp" ]; then
            eval "resp=\"\$$prompt_out\""
        fi
        if "$prompt_cmd" "$resp" "$@"; then
            eval "$prompt_out=\"\$resp\""
            return 0
        fi
    done
}

prompt_confirm() {
    while true; do
        printf "%s " "$1"
        read confirm_resp </dev/tty
        case "$confirm_resp" in
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
update=

while getopts "d:v:t:ush" opt; do
    case $opt in
        d) install_dir="$OPTARG" ;;
        v) version="$OPTARG" ;;
        t) toolchain_name="$OPTARG" ;;
        u) update=1 ;;
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

shift $((OPTIND - 1))
if [ $# -gt 0 ]; then
    echo "$0: illegal argument: $1" >&2
    print_help >&2
    exit 1
fi

triple="$(get_triple)"
no_ask="${no_ask:-0}"
install_dir="${install_dir:-$(get_default_install_dir $no_ask)}"
version="${version:-cdm-nightly}"

if [ "$no_ask" -eq 0 ] && ! true 2>/dev/null >/dev/tty; then
    echo "Cannot run in interactive mode without a controlling terminal." >&2
    exit 1
fi

check_tools rustup curl bash

if [ "$no_ask" -eq 0 ]; then
    prompt_input "Enter release name ($version):" version true
    prompt_input "Enter install directory ($install_dir):" install_dir check_install_dir update upd_toolchain_name
    toolchain_name="${toolchain_name:-$upd_toolchain_name}"
    toolchain_name="${toolchain_name:-cdm}"
    prompt_input "Enter toolchain name ($toolchain_name):" toolchain_name check_rustup_toolchain "$upd_toolchain_name"
    echo
else
    update="${update:-0}"
    check_install_dir "$install_dir" update upd_toolchain_name
    toolchain_name="${toolchain_name:-$upd_toolchain_name}"
    toolchain_name="${toolchain_name:-cdm}"
    check_rustup_toolchain "$toolchain_name" "$upd_toolchain_name"
fi

echo "Host platform:            $triple"
echo "Toolchain version:        $version"
echo "Install directory:        $install_dir"
echo "Rustup toolchain name:    $toolchain_name"
if [ "$update" -ne 0 ]; then
    echo
    echo "The existing toolchain $install_dir will be uninstalled."
fi
echo
if [ "$no_ask" -eq 0 ] && ! prompt_confirm "Proceed with installation? (Y/n)"; then
    echo "Installation canceled."
    exit 0
fi

if [ "$update" -ne 0 ]; then
    echo "Uninstalling existing toolchain..."
    "$install_dir/cdm-uninstall.sh" -s
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
curl --proto "=https" --tlsv1.2 -#fLo "$download_dir/rustc.tar.gz" "$download_url/$version/rustc-nightly-$triple.tar.gz"
echo "Downloading rust-std..."
curl --proto "=https" --tlsv1.2 -#fLo "$download_dir/rust-std.tar.gz" "$download_url/$version/rust-std-nightly-$triple.tar.gz"
echo "Downloading rust-src..."
curl --proto "=https" --tlsv1.2 -#fLo "$download_dir/rust-src.tar.gz" "$download_url/$version/rust-src-nightly.tar.gz"

mkdir -p "$install_dir"
install_dir_abs=$(cd "$install_dir" && pwd -P)

components="rustc rust-std rust-src"
for comp in $components; do
    mkdir -p "$download_dir/$comp"
    echo "Extracting $comp..."
    tar -xzf "$download_dir/$comp.tar.gz" -C "$download_dir/$comp" --strip-components=1
    echo "Copying $comp..."
    "$download_dir/$comp/install.sh" --prefix="$install_dir" >/dev/null 2>/dev/null
done

echo "Linking CDM-16 toolchain..."
rustup toolchain link "$toolchain_name" "$install_dir"

echo "Installing nightly toolchain..."
rustup toolchain install nightly

toolchain_info="$install_dir/.cdm-rust"
uninstall_script="$install_dir/cdm-uninstall.sh"

echo "Writing toolchain info..."
cat >"$toolchain_info" <<EOF
_version="$version"
_toolchain_name="$toolchain_name"
EOF

echo "Writing uninstall script..."
cat >"$uninstall_script" <<EOF
#!/bin/sh

set -e

print_help() {
    echo "CDM-16 Rust toolchain uninstall script"
    echo "Usage: \$0 [-s] [-h]"
    echo " -s : Script mode. Don't ask interactive questions."
    echo " -h : Display this help message."
}

prompt_confirm() {
    while true; do
        printf "%s " "\$1"
        read resp </dev/tty
        case "\$resp" in
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

install_dir="$install_dir_abs"
toolchain_name="$toolchain_name"
no_ask=0

while getopts "sh" opt; do
    case \$opt in
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

shift \$((OPTIND - 1))
if [ \$# -gt 0 ]; then
    echo "\$0: illegal argument: \$1" >&2
    print_help >&2
    exit 1
fi

if [ "\$no_ask" -eq 0 ] && ! true 2>/dev/null >/dev/tty; then
    echo "Cannot run in interactive mode without a controlling terminal." >&2
    exit 1
fi

echo "Toolchain \$toolchain_name will be unregistered."
echo "Directory \$install_dir will be DELETED."
echo
if [ "\$no_ask" -eq 0 ] && ! prompt_confirm "Proceed with uninstallation? (Y/n)"; then
    echo "Uninstallation canceled."
    exit 0
fi

uninstall_success=0
on_exit(){
    if [ "\$uninstall_success" -eq 0 ]; then
        echo "Uninstallation failed."
    fi
}
trap on_exit EXIT

echo "Unregistering rustup toolchain..."
if command -v rustup >/dev/null; then
    rustup toolchain uninstall "\$toolchain_name"
else
    echo "Warning: the toolchain has not been unregistered because rustup is unavailable."
fi

echo "Removing toolchain files..."
cd /
rm -rf "\$install_dir"

echo "Uninstallation completed successfully."
uninstall_success=1
EOF
chmod a+x "$uninstall_script"

echo "Uninstall script written to $uninstall_script"

echo "Installation completed successfully."
install_success=1
