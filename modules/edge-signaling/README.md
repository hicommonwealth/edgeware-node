# edge_signaling
This module contains the logic that powers Edgeware's governance UI. It is presented as a broader governance module that forms something akin to a forum for signaling proposals. Users can submit proposals, vote on proposals, and track progress of proposals through Edgeware's governance process.

## Functionality

The module exposes 2 public functions:
* `create_proposal`, which allows submission of a new governance proposal for the purpose of `Signaling`.
* `advance_proposal`, which allows the author of a proposal to shift the proposal's state, either starting or completing the voting process.

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

# Signalling Lifecycle
This module enables one to create signalling proposals and vote on them. This is useful for engaging parts of the community and understanding how the community reacts to a given idea before putting it forth in a state-changing proposal through the main governance mechanism.

The lifecycle for using this module is:
1. Create proposals
2. Vote on proposals
3. Engage in off-chain discussion


## Proposal Lifecycle
Proposals go through the lifecycle that votes go through from the [edge-voting](modules/edge-voting) module. These specific stages are described there.
1. PreVoting
2. Voting
3. Completed
