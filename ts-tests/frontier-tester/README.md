## Edgeware Frontier Tester

Unit tests for Edgeware Frontier. 

You'll need to run a frontier-compatible Edgeware node with dev signing enabled.
You can use [jake.frontier-up-2](https://github.com/hicommonwealth/edgeware-node/tree/jake.frontier-up-2).
Start the Edgeware node with the following command.

```
./target/release/edgeware --dev --enable-dev-signer
```

To run all included frontier tests, use the following:

```
yarn test web3tests
```

To run a specific test:

```
yarn test web3tests/[testName]
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
- Contract Interfaces
- Edgeware Lockdrop
- Modifiers
- Transferring balance into EVM pallet
- Timestamps
- Contract creation with non-zero contract balance