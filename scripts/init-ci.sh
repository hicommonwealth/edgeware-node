#!/usr/bin/env bash

if [[ "$OSTYPE" == "linux-gnu" ]]; then
    echo "Found linux"
    # check for root
    SUDO_PREFIX=''
    if [[ $EUID -ne 0 ]]; then
        echo "Running apt as sudo"
        SUDO_PREFIX='sudo'
    fi
    $SUDO_PREFIX apt update
    $SUDO_PREFIX apt install -y curl build-essential cmake pkg-config libssl-dev openssl git clang libclang-dev
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Found macbook"
    brew install cmake pkg-config openssl git llvm
fi

if [[ $(cargo --version) ]]; then
    echo "Found cargo"
else
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env
    export PATH=$HOME/.cargo/bin:$PATH
fi

rustup toolchain add $WASM_BUILD_TOOLCHAIN
rustup default $WASM_BUILD_TOOLCHAIN
rustup target add wasm32-unknown-unknown --toolchain $WASM_BUILD_TOOLCHAIN
