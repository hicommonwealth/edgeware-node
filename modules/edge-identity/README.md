# edge_identity
The identity module currently handles the registration, attestation, and verification of external identities on Edgeware. The types of identities one may be interested in registering through this module include other blockchain addresses or public keys, Github usernames, email addresses, and even phone numbers. The goal of this module is to be as extensible to external identities in the real or blockchain world to interface with Edgeware.


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

# Identity Lifecycle

Identities on Edgeware go through a simple state machine:

1. `Registered`
2. `Attested`
3. `Verified`

## Registration

An external identity on Edgeware first starts at the registration process. A user must possess a valid Edgeware public key and be able to sign and submit a transaction to an eligible Edgeware node. To register an identity, we only require submitting the identity formatted as a byte array `Vec<u8>` to the `register` function. Once the transaction is included in the Edgeware chain, this identity becomes `Registered`.

## Attested

An attestation of an external identity on Edgeware is a convincing proof that the registrar of the identity actually owns said identity. We denote the registrar as the possessor of the Edgeware private key that submitted the registration of the external identity. If the attester and registrar are the same person, they should have no problem convincing the active set of `verifiers` that they in fact control the external identity. The attestation is formatted as a byte array `Vec<u8>`, which should reliably point to a URL or other satisfiable proof of control.

The sender of both the registration and attestation must be the same. This helps mitigate false attestations. Therefore, the blockchain serves as the single source of truth for attestations on identity registrations. A verifier should only consult the proof stored in the chain to decide whether it is valid or not and not any other form or presentation of an attestation.

Examples of attestations for different settings:

1. **Github**

    The identity is a Github `username`. The registrar, upon registering the username, should publish a Gist under the `username`'s account with a link to the registration on Edgeware.

2. **Ethereum**

    The identity is an Ethereum public key or address. The registrar, upon registering the username, could send a 0 value transaction to a burn address symbolizing the public key of their corresponding Edgeware account and submit the transaction hash as the attestation. Upon inspection, a verifier should have enough proof that if the owner of the Ethereum account did not also own the recipient Edgeware account (represented as the target address on Ethereum), then they would not issue such a transaction.

## Verified

The final and permanent state of an identity on Edgeware as it stands is a verified identity. Since individuals can still register and attest to identities they don't control, by submitting false attestation proofs, they should not be able to fool the active set of verifiers. The job of the active verifiers is to check attestation proofs and vote for or against a verification.

A verification of an identity is accepted or rejected once 2/3 of the active verifiers vote for the respective outcome. Once accepted, identities remain verified forever unless future governance procedures are developed to change the logic.