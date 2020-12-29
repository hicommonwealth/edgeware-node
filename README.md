# edgeware-node

[Edgeware](https://edgewa.re) is an:
- On-chain Governed,
- Proof-of-Stake (PoS) Blockchain
- with a WASM Runtime.

## For node operators, validators, and other users

A getting started guide can be found at our [Github
Wiki](https://github.com/hicommonwealth/edgeware-node/wiki), including
guides for running a node, validating, and setting up basic monitoring
tools to keep your node online.

For more details about the project, visit the Edgeware website, or
check out the [blog](https://blog.edgewa.re) or
[Twitter](https://twitter.com/heyedgeware). Finally, for discussion and
governance, campaigns and proposals can be found on
[Commonwealth](https://commonwealth.im).

## For developers

### Quickstart

If your device is clean (such as a fresh cloud VM) you can use this
script for an automated setup:

```
./setup.sh
```

Otherwise, proceed with the full instructions below.

### Manual setup

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
```

Build Edgeware:

```
cargo build --release
```

Ensure you have a fresh start if updating from another version:
```
./scripts/purge-chain.sh <NETWORK_NAME_ID>
```
To start up the Edgeware node and connect to the Mainnet, run:
```
./target/release/edgeware --chain=edgeware --name <INSERT_NAME> --wasm-execution Compiled
```
To start up the Edgeware node and connect to the Beresheet testnet, run:
```
./target/release/edgeware --chain=beresheet --name <INSERT_NAME>
```

### Docker

To make it easier run Edgeware node on OSX, we have script that will use right `.dockerignore.remotecargo`, use binary we have compiled in `/target/release`, build image and then run it. You can read more about [Remote Cargo](https://medium.com/@yangWao/build-substrate-in-few-minutes-with-fraction-costs-26fce6aa5066?sk=514e94de6c702b92f5078860623168cc), what we are doing to use this script.

```
./build_and_run_devnet.sh
```
It will on background create symlink from `.dockerignore.remotecargo` to `.dockerignore`

If you previously build image `cwl/edgeware`, you can use docker-compose as well
```
docker-compose up edgeware
```
You will have exposed ports 9933,9944 and 30333.

### Module Benchmarking

To build in benchmarking mode:
```
cd node/cli && cargo build --features runtime-benchmarks --release
```

To run benchmarks and output new weight files while still in the `node/cli` folder (replace `signaling` with `voting` to benchmark voting instead):
```
../../target/release/edgeware benchmark --pallet signaling --extrinsic "*" --steps 50 --repeat 20 --output ../runtime/src/weights/
```
If the amount of time it takes to run the benchmark is too long, consider reducing the `steps` and `repeat` parameters.