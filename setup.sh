#!/usr/bin/env bash

./scripts/init.sh

RELEASE=""
if [ -z "$DEBUG_BUILD" ]; then
    RELEASE="--release"
fi

cargo build $RELEASE
