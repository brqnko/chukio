#!/bin/sh

# Exit on command failures and references to unset variables.
set -eu

# Resolve paths relative to the repository, regardless of the caller's directory.
root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

# Print the supported commands and common examples.
usage() {
    cat >&2 <<'EOF'
Usage:
    ./x test [cargo-test-args...]
    ./x fmt [cargo-fmt-args...]
    ./x clippy [cargo-clippy-args...]
    ./x mir <name-or-path>

Examples:
    ./x test
    ./x test interprets_calc
    ./x fmt
    ./x fmt -- --check
    ./x clippy
    ./x clippy -- -D warnings
    ./x mir calc
    ./x mir chukio/tests/ios/calc.rs
EOF
}

command=${1-}
case "$command" in
    test)
        shift
        cd "$root"
        export RUST_BACKTRACE=1
        # Append --nocapture after an existing separator, or add the separator ourselves.
        for arg in "$@"; do
            if [ "$arg" = "--" ]; then
                exec cargo test --workspace "$@" --nocapture
            fi
        done
        exec cargo test --workspace "$@" -- --nocapture
        ;;
    fmt)
        shift
        cd "$root"
        exec cargo fmt --all "$@"
        ;;
    clippy)
        shift
        cd "$root"
        exec cargo clippy --workspace --all-targets "$@"
        ;;
    mir)
        if [ "$#" -ne 2 ]; then
            usage
            exit 2
        fi
        ;;
    *)
        usage
        exit 2
        ;;
esac

# Accept absolute paths, repository-relative Rust files, or fixture names.
input=$2
case "$input" in
    /*)
        source=$input
        ;;
    *.rs)
        if [ -f "$root/$input" ]; then
            source=$root/$input
        else
            source=$root/chukio/tests/ios/$input
        fi
        ;;
    *)
        source=$root/chukio/tests/ios/$input.rs
        ;;
esac

if [ ! -f "$source" ]; then
    echo "error: MIR input not found: $input" >&2
    exit 1
fi

cd "$root"
# Emit unoptimized MIR that stays close to the source program.
exec rustc \
    --edition=2024 \
    -Copt-level=0 \
    -Zmir-opt-level=0 \
    -Coverflow-checks=yes \
    -Zunpretty=mir \
    "$source"
