#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

filter_known_runtime_noise() {
    grep -v -F "Error watching for xdg color schemes:"
}

exec cargo run 2> >(filter_known_runtime_noise >&2)
