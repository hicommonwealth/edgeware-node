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

    # Setup Docker CE. See https://github.com/ltfschoen/polkadot-linode/blob/master/setup-docker.sh
    apt-get install -y apt-transport-https ca-certificates curl software-properties-common;
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -;
    add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu bionic test";
    apt-get -y update && apt-get -y upgrade;
    apt-get install -y docker-ce docker-compose;
    docker --version;
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
