#!/usr/bin/env bash

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	echo "Found linux"
  # check for root
  SUDO_PREFIX=''
  if [[ $EUID -ne 0 ]]; then
      echo "Running apt as sudo"
      SUDO_PREFIX='sudo'
  fi
	$SUDO_PREFIX apt install cmake pkg-config libssl-dev git clang libclang-dev
elif [[ "$OSTYPE" == "darwin"* ]]; then
	echo "Found macbook"
	brew install cmake pkg-config openssl git llvm
fi

if [[ $(cargo --version) ]]; then
	echo "Found cargo"
else
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	source $HOME/.cargo/env  
	rustup update nightly
	rustup target add wasm32-unknown-unknown --toolchain nightly
	rustup update stable
fi
	
if [[ $(wasm-gc) ]]; then
	echo "Found wasm-gc"
else
	cargo install --git https://github.com/alexcrichton/wasm-gc
fi

RELEASE=""
if [ -z "$DEBUG_BUILD" ]; then
    RELEASE="--release"
fi

cargo build $RELEASE
cd subkey && cargo build --release
