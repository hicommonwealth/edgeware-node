#!/usr/bin/env bash

if [[ $(cargo --version) ]]; then
	echo "Found cargo"
else
	curl https://sh.rustup.rs -sSf | sh
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

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	echo "Found linux"
	sudo apt install cmake pkg-config libssl-dev git clang libclang-dev
elif [[ "$OSTYPE" == "darwin"* ]]; then
	echo "Found macbook"
	brew install cmake pkg-config openssl git llvm
fi

cargo build --release
cd subkey && cargo build --release