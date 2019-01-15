# edge_voting
This module contains logic for voting. It consumes the delegation module to tally votes and should similarly be consumed by all other modules that want to handle voting. It currently supports binary and multi-option elections with optional commit/reveal schemes using the Blake2Hash function as the hashing algorithm.

# Setup
Install rust or update to the latest versions.
```
curl https://sh.rustup.rs -sSf | sh
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup update stable
cargo install --git https://github.com/alexcrichton/wasm-gc
```

You will also need to install the following packages:

Linux:
```
sudo apt install cmake pkg-config libssl-dev git clang libclang-dev
```

Mac:
```
brew install cmake pkg-config openssl git llvm
```
