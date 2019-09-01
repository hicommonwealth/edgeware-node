#!/usr/bin/env bash

./scripts/init.sh

RELEASE=""
if [ -z "$DEBUG_BUILD" ]; then
    RELEASE="--release"
fi

source $HOME/.cargo/env

# `cargo +nightly-2019-06-30 build --release` to override the default
# version of cargo/rust being used with a specific version, for
# if version of rust nightly has issues.
# or switch first with `rustup default nightly-2019-06-30`
cargo build $RELEASE

# Check that your Cargo.lock file is the same as upstream and also
# rust is up to date each time

# Do not run `cargo update`