# edge_voting
This module contains logic for voting. It currently supports binary and multi-option elections with optional commit/reveal schemes using the Blake2Hash function as the hashing algorithm.

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

# Voting Lifecycle
Votes go through a number of stages, conditional on the type of vote.
1. Prevoting
2. (Optional) Commit
3. Voting
4. Completed

## Tally types
- One person, one vote
- One coin, one vote

## Voting types
- Binary votes
- Multi-option votes
- Commit-reveal votes

## Prevoting
The prevoting stage marks the creation of a vote. Additionally, in this stage no voting can take place. This is currently being used by the [edge-signaling](modules/edge-signaling) module.

## Commit
The commit stage is used for votes that require commit-reveal schemes. Within this stage, all participants submit commitments. After the commit phase, all participants should reveal.

## Voting
The voting stage doubles as a reveal phase when the vote uses a commit-reveal scheme and simply a general public vote otherwise.

## Completed
The completed stage marks the ending of a vote, meaning no further votes will be considered in a tally.