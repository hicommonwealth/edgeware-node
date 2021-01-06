#!/usr/bin/env bash

./scripts/init.sh

RELEASE=""
if [ -z "$DEBUG_BUILD" ]; then
    RELEASE="--release"
fi

source $HOME/.cargo/env

cargo build $RELEASE
