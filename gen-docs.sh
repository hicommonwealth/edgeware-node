#!/bin/bash
set -e

export CARGO_TARGET_DIR=target

for toml in $(find . -maxdepth 3 -name "Cargo.toml"); do
    cargo update --manifest-path $toml || true
    cargo doc --no-deps --manifest-path $toml "$@"
done
