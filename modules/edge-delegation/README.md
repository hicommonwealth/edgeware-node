# edge_delegation
This module contains the basic delegation interface that should be consumed by other modules. Accounts can delegate their unit voting preference to another account or delegator. When an election/ballot is terminating in another module of Edgeware, we can receive a tally of the most up to date delegators that are sinks along any account's delegation path. For each account, the up to date delegator indicates where to allocate such an account's votes.

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
sudo apt install cmake pkg-config libssl-dev git
```

Mac:
```
brew install cmake pkg-config openssl git
```