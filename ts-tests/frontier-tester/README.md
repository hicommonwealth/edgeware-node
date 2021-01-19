## Edgeware Frontier Tester

Unit tests for Edgeware Frontier. You must have the node built to run these tests. The location of the binary is defined in the environment variable `BINARY_PATH`, whose default is set in [utils.js](contracts/utils.js) as `../../target/release/edgeware`.

To run a single test, use:

```
yarn test tests/<testName.js/ts>
```

To run all included frontier tests, use the following:

```
yarn test-ci
```

The following functionality is tested:
- Adding Liquidity to a fresh Uniswap deployment
- Generating an ERC20 Token Allowance
- Create Factory Contract 
- Create2 Factory Contract 
- Calling a precompile (ECRecover)
- Event emission and subscription
- Fallback function
- Hashing (on chain and with web3 provider): keccak256, sha3, ripemd
- Identity, modexp, and ed25519 precompiles.
- Contract Interfaces
- Edgeware Lockdrop
- Modifiers
- Transferring balance into EVM pallet
- Timestamps
- Contract creation with non-zero contract balance
- Basic substrate-native functionality including democracy proposals, treasury proposals, and identity registration.