# edge_governance
This module contains the logic that powers Edgeware's governance UI. It is presented as a broader governance module that forms something akin to a forum for governance proposals. Users can submit proposals, vote on proposals, and track progress of proposals through Edgeware's governance process.

## Functionality

The module exposes 4 public functions:
* `create_proposal`, which allows submission of a new governance proposal, for the purpose of `Funding`, a chain `Upgrade`, or `Signaling`.
* `add_comment`, which attaches a new comment to an existing proposal.
* `advance_proposal`, which allows the author of a proposal to shift the proposal's state, either starting or completing the voting process.
* `submit_vote`, which allows a user to place their vote.

## Setup

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

Once done with setup, run `cargo build` to compile the module and `cargo test` to run our unit tests.