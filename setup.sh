#!/usr/bin/env bash

./scripts/init.sh

RELEASE=""
if [ -z "$DEBUG_BUILD" ]; then
    RELEASE="--release"
fi

source $HOME/.cargo/env

git submodule update --init --recursive

cargo build $RELEASE
