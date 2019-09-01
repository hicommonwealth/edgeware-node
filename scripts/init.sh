#!/usr/bin/env bash

if [[ "$OSTYPE" == "linux-gnu" ]]; then
    echo "Found linux"
    # check for root
    SUDO_PREFIX=''
    if [[ $EUID -ne 0 ]]; then
        echo "Running apt-get as sudo"
        SUDO_PREFIX='sudo'
    fi
    $SUDO_PREFIX apt-get update
    $SUDO_PREFIX apt-get install -y build-essential cmake pkg-config libssl-dev openssl git clang libclang-dev
    $SUDO_PREFIX apt-get install -y vim unzip screen sudo
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Found macbook"
    brew install cmake pkg-config openssl git llvm
fi

if [[ $(cargo --version) ]]; then
    echo "Found cargo"
else
    # wget -O - https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2019-06-30
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env
    export PATH=$HOME/.cargo/bin:$PATH
fi

# `rustup uninstall` if any Git issues
rustup update stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo --version

if [[ $(wasm-gc) ]]; then
    echo "Found wasm-gc"
else
    cargo install --git https://github.com/alexcrichton/wasm-gc
fi
