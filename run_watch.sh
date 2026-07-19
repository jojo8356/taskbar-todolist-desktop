#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

if ! command -v cargo-watch >/dev/null 2>&1; then
    echo "cargo-watch is not installed." >&2
    echo "Install it with: cargo install cargo-watch" >&2
    exit 127
fi

filter_known_runtime_noise() {
    grep -v -F "Error watching for xdg color schemes:"
}

exec cargo watch \
    --clear \
    --why \
    --watch src \
    --watch Cargo.toml \
    --watch Cargo.lock \
    --exec "run" \
    2> >(filter_known_runtime_noise >&2)
