# edgeware-node

A Parity Substrate node implementing [Edgeware](https://edgewa.re).

## Implemented Modules

### Edge

* [Identity](modules/edge-identity)
* [Governance](modules/edge-governance)
* [Voting](modules/edge-voting)
* [Delegation](modules/edge-delegation)

### SRML

* [Aura](https://github/com/paritytech/substrate/tree/master/srml/aura)
* [Balances](https://github/com/paritytech/substrate/tree/master/srml/balances)
* [Consensus](https://github/com/paritytech/substrate/tree/master/srml/consensus)
* [Contract](https://github/com/paritytech/substrate/tree/master/srml/contract)
* [Council](https://github/com/paritytech/substrate/tree/master/srml/council)
* [Democracy](https://github/com/paritytech/substrate/tree/master/srml/democracy)
* [Executive](https://github/com/paritytech/substrate/tree/master/srml/executive)
* [Fees](https://github/com/paritytech/substrate/tree/master/srml/fees)
* [Finality_tracker](https://github/com/paritytech/substrate/tree/master/srml/finality-tracker)
* [Grandpa](https://github/com/paritytech/substrate/tree/master/srml/grandpa)
* [Indices](https://github/com/paritytech/substrate/tree/master/srml/indices)
* [Session](https://github/com/paritytech/substrate/tree/master/srml/session)
* [Staking](https://github/com/paritytech/substrate/tree/master/srml/staking)
* [System](https://github/com/paritytech/substrate/tree/master/srml/system)
* [Timestamp](https://github/com/paritytech/substrate/tree/master/srml/timestamp)
* [Treasury](https://github/com/paritytech/substrate/tree/master/srml/treasury)
* [Sudo](https://github/com/paritytech/substrate/tree/master/srml/sudo)

## Adding A Module

1. Add its github repo to:
  - [Cargo.toml](Cargo.toml)
  - [node/runtime/Cargo.toml](node/runtime/Cargo.toml)
  - [node/runtime/wasm/Cargo.toml](node/runtime/wasm/Cargo.toml) (be sure to have `default-features = false`)
2. Changes to [the runtime](node/runtime/src/lib.rs):
  - Add it as an `extern crate`.
  - Implement its `Trait` with production types.
  - Add it to the `construct_runtime` macro with all implemented components.
3. If its storage contains `config` elements, then you need to modify [the chain spec](node/src/chain_spec.rs):
  - Add it to the `edgeware_runtime`'s list of `Config` types.
  - Add it to the `testnet_genesis` function, initializing all storage fields set to `config()`.
4. Build and run the chain.

## Usage
The fastest setup is to run the following script. It will build the release version using the pre-compiled binary:
```
./setup.sh
```
Then proceed to the *Running* instructions or follow the instructions below for the manual setup.

### Initial Setup

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

### Building

```
./build.sh
cargo build --release
```

### Running

Ensure you have a fresh start if updating from another version:
```
./purge-chain.sh
```
To start up the Edgeware node and connect to the latest testnet, run:
```
./target/release/edgeware --chain=edgeware --name <INSERT_NAME>
```

If you use the `--key` flag, ensure that either it is a 32-byte hex string or prefixed with `//` like so:
```
./target/release/edgeware --chain=edgeware --name <INSERT_NAME> --key //testkey
```

### Visualization

To ensure you followed the steps correct, check https://telemetry.polkadot.io/#/Edgeware%20V0.1.7. If done correctly, you should see your node with the inserted name.
