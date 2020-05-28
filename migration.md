# Edgeware Migration
Based on [PRs with migrations hackMD](https://hackmd.io/p97ldDxfST6WF-1m-MxwOA).

## Migration PRs
+ [unique storage names](https://github.com/paritytech/substrate/pull/5010/)
+ [iterable storage maps (away from opaque hashes)](https://github.com/paritytech/substrate/pull/5226)
+ [democracy redesign](https://github.com/paritytech/substrate/pull/5294/)
+ [prime member selection](https://github.com/paritytech/substrate/pull/5346)

### Removed Migrations
+ https://github.com/paritytech/substrate/pull/5870
+ https://github.com/paritytech/substrate/pull/5291
+ https://github.com/paritytech/substrate/pull/5224

## Pallets
+ System: frame_system
+ Utility: pallet_utility
+ Aura: pallet_aura

+ Timestamp: pallet_timestamp
+ Authorship: pallet_authorship
+ Indices: pallet_indices
+ Balances: pallet_balances
+ TransactionPayment: pallet_transaction_payment

+ Staking: pallet_staking
+ Session: pallet_session
+ Democracy: pallet_democracy
+ Council: pallet_collective::<Instance1>
+ Elections: pallet_elections_phragmen

+ FinalityTracker: pallet_finality_tracker
+ Grandpa: pallet_grandpa
+ Treasury: pallet_treasury
+ Contracts: pallet_contracts

+ Sudo: pallet_sudo
+ ImOnline: pallet_im_online
+ AuthorityDiscovery: pallet_authority_discovery
+ Offences: pallet_offences
+ RandomnessCollectiveFlip: pallet_randomness_collective_flip
+ Identity: pallet_identity
+ Scheduler: pallet_scheduler
+ Vesting: pallet_vesting
+ EVM: pallet_evm
+ Historical: pallet_session_historical

+ Signaling: signaling
+ Voting: voting
+ TreasuryReward: treasury_reward