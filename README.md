# edgeware-node

[Edgeware](https://edgewa.re) is an:
- On-chain Governed,
- Proof-of-Stake (PoS) Blockchain
- with a WASM Runtime.

Documentation can be found at our [Github Wiki](https://github.com/hicommonwealth/edgeware-node/wiki). For more details about the project, visit the Edgeware website, or check the [blog](blog.edgewa.re) or [Twitter](http://twitter.com/heyedgeware) for the latest. Finally, for discussion and governance, you can use [Commonwealth.im](https://commonwealth.im).

### Quickstart

If your device is clean (such as a fresh cloud VM) you can use this
script for an automated setup:

```
./setup.sh
```

Otherwise, proceed with the full instructions below.

Install system dependencies:

Linux:
```
sudo apt install cmake pkg-config libssl-dev git clang libclang-dev
```

Mac:
```
brew install cmake pkg-config openssl git llvm
```

Install Edgeware dependencies:

```
curl https://sh.rustup.rs -sSf | sh
rustup update stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install --git https://github.com/alexcrichton/wasm-gc
```

Build Edgeware:

```
cargo build --release
```

Ensure you have a fresh start if updating from another version:
```
./scripts/purge-chain.sh
```

To start up the Edgeware node and connect to testnet 0.9.0, run:
```
./target/release/edgeware --chain=chains/testnet-0.9.0.json --name <INSERT_NAME>
```
