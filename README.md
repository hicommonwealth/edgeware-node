# edgeware-node

A Parity Substrate node implementing our edgeware-related modules.

## Implemented Modules

### Edge

* [Identity](https://github.com/hicommonwealth/edge_identity)
* [Bridge](https://github.com/hicommonwealth/edge_bridge)
* [Delegation](https://github.com/hicommonwealth/edge_delegation)

### SRML

* [Timestamp](https://github.com/paritytech/substrate/tree/master/srml/timestamp)
* [Consensus](https://github.com/paritytech/substrate/tree/master/srml/consensus)
* [Balances](https://github.com/paritytech/substrate/tree/master/srml/balances)
* [Session](https://github.com/paritytech/substrate/tree/master/srml/session)
* [UpgradeKey](https://github.com/paritytech/substrate/tree/master/srml/upgrade-key)

## Adding A Module

1. Add its github repo to:
  - [Cargo.toml](Cargo.toml)
  - [runtime/Cargo.toml](runtime/Cargo.toml)
  - [runtime/wasm/Cargo.toml](runtime/wasm/Cargo.toml) (be sure to have `default-features = false`)
2. Changes to [the runtime](runtime/src/lib.rs):
  - Add it as an `extern crate`.
  - Implement its `Trait` with production types.
  - Add it to the `construct_runtime` macro with all implemented components.
3. If its storage contains `config` elements, then you need to modify [the chain spec](src/chain_spec.rs):
  - Add it to the `edgeware_runtime`'s list of `Config` types.
  - Add it to the `testnet_genesis` function, initializing all storage fields set to `config()`.
4. Build and run the chain.
5. (Optional) If using new types, add them to the API options in [Edge Api](https://github.com/hicommonwealth/edge_api).

## Usage

### Initial Setup

```
curl https://sh.rustup.rs -sSf | sh
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup update stable
cargo install --git https://github.com/alexcrichton/wasm-gc
sudo apt install cmake pkg-config libssl-dev git clang libclang-dev
```

### Building

```
./build.sh
cargo build --release
```

### Running

If you've rebuilt the runtime and are using the default development chain storage location (`~/.local/share/Substrate/chains/development/`), run the `./purge-chain.sh` script to clear your old chain's history.

```
./target/release/edgeware --dev
```
