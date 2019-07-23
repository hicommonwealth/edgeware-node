# edgeware-node

A Parity Substrate node implementing [Edgeware](https://edgewa.re).

## To get started

- Download this entire repository to the file system that you are using to run the validator node.
  - You can do this by going to [this page](https://github.com/hicommonwealth/edgeware-node) and selecting "Clone or download" followed by "Download ZIP".
  - If you are installing via a command line interface (e.g. SSH into a remote server), you can dow-load by running `wget https://github.com/hicommonwealth/edgeware-node/archive/master.zip`
  - Once you have downloaded the zip file, unzip the `edgeware-node-master` folder onto the file system. If you are using a command line interface, you can unzip by running `unzip master.zip`
  - **_All commands referenced in this document need to be run from within the `edgeware-node-master` folder._**

- You will also need to install `rust` and `cargo` by installing `rustup` [here](https://rustup.rs/).
  - **_Note_**: at the end of the install, you will need to log out and log in again, or run the suggested `source` command to configure the current shell.

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
rustup update stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
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
./scripts/build.sh
cargo build --release
```

### Running

Ensure you have a fresh start if updating from another version:
```
./scripts/purge-chain.sh
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

* [Identity](https://github.com/hicommonwealth/edgeware-node/tree/master/modules/edge-identity)
* [Voting](https://github.com/hicommonwealth/edgeware-node/tree/master/modules/edge-voting)
* [Governance](https://github.com/hicommonwealth/edgeware-node/tree/master/modules/edge-governance)

### SRML
* [System](https://github.com/paritytech/substrate/tree/master/srml/system)
* [Aura](https://github.com/paritytech/substrate/tree/master/srml/aura)
* [Timestamp](https://github.com/paritytech/substrate/tree/master/srml/timestamp)
* [Authorship](https://github.com/paritytech/substrate/tree/master/srml/authorship)
* [Indices](https://github.com/paritytech/substrate/tree/master/srml/indices)
* [Balances](https://github.com/paritytech/substrate/tree/master/srml/balances)
* [Staking](https://github.com/paritytech/substrate/tree/master/srml/staking)
* [Session](https://github.com/paritytech/substrate/tree/master/srml/session)
* [Democracy](https://github.com/paritytech/substrate/tree/master/srml/democracy)
* [Council](https://github.com/paritytech/substrate/tree/master/srml/council)
* [Elections](https://github.com/paritytech/substrate/tree/master/srml/elections)
* [FinalityTracker](https://github.com/paritytech/substrate/tree/master/srml/finality-tracker)
* [Grandpa](https://github.com/paritytech/substrate/tree/master/srml/grandpa)
* [Treasury](https://github.com/paritytech/substrate/tree/master/srml/treasury)
* [Contracts](https://github.com/paritytech/substrate/tree/master/srml/contracts)
* [Sudo](https://github.com/paritytech/substrate/tree/master/srml/sudo)

## Developing on Edgeware

### Running A Local Chain

To run a chain locally for development purposes: `./target/release/edgeware --chain=local --alice --validator`

To allow apps in your browser to connect, as well as anyone else on your local network, add the `--rpc-cors=all` flag.

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
