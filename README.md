# edgeware-node

A Parity Substrate node implementing [Edgeware](https://edgewa.re).

## To get started

Download this entire repository to the file system that you are using to run the validator node.

You can do this by going to [this page](https://github.com/hicommonwealth/edgeware-node) and selecting "Clone or download" followed by "Download ZIP".

Once you have downloaded the zip file, unzip the `edgeware-node-master` folder onto the file system. All commands referenced in this document need to be run from within this `edgeware-node-master` folder.

## Fresh start
If your device is clean (such as a fresh cloud VM) you can use this script, otherwise, proceed with the *Initial Setup*.
```
./setup.sh
```
To create a keypair, install subkey with `cargo install --force --git https://github.com/paritytech/substrate subkey`. Then run the following:
```
subkey generate
```
To create an ED25519 keypair, run the following:
```
subkey -e generate
```
To create derived keypairs, use the mnemonic generated from a method above and run:
```
subkey inspect "<mnemonic>"//<derive_path>
```
For example:
```
subkey inspect "west paper guide park design weekend radar chaos space giggle execute canoe"//edgewarerocks
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

To ensure you followed the steps correctly, check https://telemetry.polkadot.io/#/Edgeware%20Testnet%20V0.2.0. If done correctly, you should see your node with the inserted name.

## Implemented Modules

### Edge

* [Identity](modules/edge-identity)
* [Governance](modules/edge-governance)
* [Voting](modules/edge-voting)

### SRML

* [Aura](https://github.com/paritytech/substrate/tree/master/srml/aura)
* [Balances](https://github.com/paritytech/substrate/tree/master/srml/balances)
* [Contracts](https://github.com/paritytech/substrate/tree/master/srml/contracts)
* [Council](https://github.com/paritytech/substrate/tree/master/srml/council)
* [Democracy](https://github.com/paritytech/substrate/tree/master/srml/democracy)
* [Executive](https://github.com/paritytech/substrate/tree/master/srml/executive)
* [Fees](https://github.com/paritytech/substrate/tree/master/srml/fees)
* [Finality_tracker](https://github.com/paritytech/substrate/tree/master/srml/finality-tracker)
* [Grandpa](https://github.com/paritytech/substrate/tree/master/srml/grandpa)
* [Indices](https://github.com/paritytech/substrate/tree/master/srml/indices)
* [Session](https://github.com/paritytech/substrate/tree/master/srml/session)
* [Staking](https://github.com/paritytech/substrate/tree/master/srml/staking)
* [System](https://github.com/paritytech/substrate/tree/master/srml/system)
* [Timestamp](https://github.com/paritytech/substrate/tree/master/srml/timestamp)
* [Treasury](https://github.com/paritytech/substrate/tree/master/srml/treasury)
* [Sudo](https://github/com.paritytech/substrate/tree/master/srml/sudo)

## Developing on Edgeware

### Running A Local Chain

To run a chain locally for development purposes: `./target/release/edgeware --chain=local --alice --validator`

To force your local to create new blocks, even if offline, add the `--force-authoring` flag.

### Adding A Module

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

