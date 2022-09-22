use super::*;

/// Migrate from `PalletVersion` to the new `StorageVersion`
pub struct MigratePalletVersionToStorageVersion;
impl OnRuntimeUpgrade for MigratePalletVersionToStorageVersion {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        frame_support::migrations::migrate_from_pallet_version_to_storage_version::<
            AllPalletsWithSystem,
        >(&RocksDbWeight::get())
    }
}

/// Phragmen deposit upgrade
const OLD_CANDIDACY_BOND: Balance = 1000 * DOLLARS;
const OLD_VOTING_BOND: Balance = 10 * DOLLARS;
pub struct PhragmenElectionDepositRuntimeUpgrade;
impl pallet_elections_phragmen::migrations::v3::V2ToV3 for PhragmenElectionDepositRuntimeUpgrade {
    type Pallet = PhragmenElection;
    type AccountId = AccountId;
    type Balance = Balance;
}
impl OnRuntimeUpgrade for PhragmenElectionDepositRuntimeUpgrade {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        pallet_elections_phragmen::migrations::v3::apply::<Self>(
            OLD_VOTING_BOND,
            OLD_CANDIDACY_BOND,
        )
    }
}

pub mod staking_migration_v5 {
	use codec::Decode;
	#[derive(Decode)]
	struct OldValidatorPrefs {
		#[codec(compact)]
		pub commission: sp_runtime::Perbill
	}
	impl OldValidatorPrefs {
		fn upgraded(self) -> pallet_staking::ValidatorPrefs {
			pallet_staking::ValidatorPrefs {
				commission: self.commission,
				.. Default::default()
			}
		}
	}

	/// Migrate storage to v5.
	/// migrate to blockable. Note that we cannot infer the storage, which is private.
	pub fn migrate<T: pallet_staking::Config>() -> frame_support::weights::Weight {
		frame_support::log::info!("Migrating staking to Releases::V5_0_0");
		pallet_staking::Validators::<T>::translate::<OldValidatorPrefs, _>(|_, p| Some(p.upgraded()));
		pallet_staking::ErasValidatorPrefs::<T>::translate::<OldValidatorPrefs, _>(|_, _, p| Some(p.upgraded()));
		frame_support::traits::StorageVersion::new(5).put::<crate::Staking>();
		0
	}
}

/// Migration of the staking pallet.
pub struct AllStakingMigrations;
impl OnRuntimeUpgrade for AllStakingMigrations {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
        let mut weight = 0;
		weight += staking_migration_v5::migrate::<crate::Runtime>();
		frame_support::log::info!(" 🦾 migration to v5 complete");
		weight += pallet_staking::migrations::v6::migrate::<crate::Runtime>();
		frame_support::log::info!(" 🦾 migration to v6 complete");
		weight += pallet_staking::migrations::v7::migrate::<crate::Runtime>();
		frame_support::log::info!(" 🦾 migration to v7 complete");
		weight += pallet_staking::migrations::v8::migrate::<crate::Runtime>();
		frame_support::log::info!(" 🦾 migration to v8 complete");
		weight += <pallet_staking::migrations::v9::InjectValidatorsIntoVoterList<Runtime> as OnRuntimeUpgrade>::on_runtime_upgrade();
		frame_support::log::info!(" 🦾 migration to v9 complete");
		weight
	}
}

/// Migration to triple reference counting.
pub struct SystemToTripleRefCount;
impl frame_system::migrations::V2ToV3 for SystemToTripleRefCount {
	type Pallet = System;
	type AccountId = AccountId;
	type Index = Index;
	type AccountData = pallet_balances::AccountData<Balance>;
}
impl OnRuntimeUpgrade for SystemToTripleRefCount {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		frame_system::migrations::migrate_from_single_to_triple_ref_count::<Self>()
	}
}

/// Remove Offences delay from storage.
pub struct OffencesDelayCleaningMigration;
impl OnRuntimeUpgrade for OffencesDelayCleaningMigration {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		pallet_offences::migration::remove_deferred_storage::<crate::Runtime>()
	}
}

const COUNCIL_OLD_PREFIX: &str = "Instance1Collective";
/// Migrate from `Instance1Collective` to the new pallet prefix `Council`
pub struct CouncilStoragePrefixMigration;
impl OnRuntimeUpgrade for CouncilStoragePrefixMigration {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		pallet_collective::migrations::v4::migrate::<crate::Runtime, Council, _>(COUNCIL_OLD_PREFIX)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		pallet_collective::migrations::v4::pre_migrate::<Council, _>(COUNCIL_OLD_PREFIX);
		Ok(())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		pallet_collective::migrations::v4::post_migrate::<Council, _>(COUNCIL_OLD_PREFIX);
		Ok(())
	}
}

const BOUNTIES_OLD_PREFIX: &str = "Treasury";
/// Migrate from 'Treasury' to the new prefix 'Bounties'
pub struct BountiesPrefixMigration;
impl OnRuntimeUpgrade for BountiesPrefixMigration {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		use frame_support::traits::PalletInfo;
		let name = <crate::Runtime as frame_system::Config>::PalletInfo::name::<Bounties>()
			.expect("Bounties is part of runtime, so it has a name; qed");
		pallet_bounties::migrations::v4::migrate::<crate::Runtime, Bounties, _>(BOUNTIES_OLD_PREFIX, name)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		use frame_support::traits::PalletInfo;
		let name = <crate::Runtime as frame_system::Config>::PalletInfo::name::<Bounties>()
			.expect("Bounties is part of runtime, so it has a name; qed");
		pallet_bounties::migrations::v4::pre_migration::<crate::Runtime, Bounties, _>(
			BOUNTIES_OLD_PREFIX,
			name,
		);
		Ok(())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		use frame_support::traits::PalletInfo;
		let name = <crate::Runtime as frame_system::Config>::PalletInfo::name::<Bounties>()
			.expect("Bounties is part of runtime, so it has a name; qed");
		pallet_bounties::migrations::v4::post_migration::<crate::Runtime, Bounties, _>(
			BOUNTIES_OLD_PREFIX,
			name,
		);
		Ok(())
	}
}

const TIPS_OLD_PREFIX: &str = "Treasury";
/// Migrate pallet-tips from `Treasury` to the new pallet prefix `Tips`
pub struct TipsPalletPrefixMigration;
impl OnRuntimeUpgrade for TipsPalletPrefixMigration {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		pallet_tips::migrations::v4::migrate::<crate::Runtime, Tips, _>(TIPS_OLD_PREFIX)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		pallet_tips::migrations::v4::pre_migrate::<crate::Runtime, Tips, _>(TIPS_OLD_PREFIX);
		Ok(())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		pallet_tips::migrations::v4::post_migrate::<crate::Runtime, Tips, _>(TIPS_OLD_PREFIX);
		Ok(())
	}
}

// Migration for scheduler pallet to move from a plain Call to a CallOrHash.
pub struct SchedulerMigrationV3;
impl OnRuntimeUpgrade for SchedulerMigrationV3 {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		Scheduler::migrate_v1_to_v3()
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		Scheduler::pre_migrate_to_v3()
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		Scheduler::post_migrate_to_v3()
	}
}

/// All Migrations for the contracts pallet, from v3 up to v7.
pub struct AllContractsMigrations;
impl OnRuntimeUpgrade for AllContractsMigrations {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		pallet_contracts::migration::migrate::<crate::Runtime>();
		0
	}
}

use sp_io::hashing::blake2_128;
/// Crediting back the w3f.
pub struct Web3AccountCreditMigration;
impl OnRuntimeUpgrade for Web3AccountCreditMigration {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		// The public key of the account of the web3 foundation to credit.
		// /!\ for now the destination account has been communicated but
		// without cryptographic guarantees, so we use this account only
		// as a placeholder.
		// "0xd96fd7613d51508eedc40cc848108290179fea6ffe8cc1d1f86631a18e0ac55f"
		// public key corresponding to the Edgeware address
		// nRiRbxgTpkyxjoQdQuJZmwwzXf25JWNDormRLWwbgukkXJP
		let web3_id = hex_literal::hex!(
			"d96fd7613d51508eedc40cc848108290179fea6ffe8cc1d1f86631a18e0ac55f"
		);
		// We do credit 150 Mo EDG to the account.
		// Here "DOLLARS" stands for 1^18 units of the lowest denomination, i.e. 1EDG.
		let to_credit = 150_000_000 * DOLLARS;
		// Overriding the storage below.
		let pfx: [u8; 32] = [38, 170, 57, 78, 234, 86, 48, 224,
							 124, 72, 174, 12, 149, 88, 206, 247,
							 185, 157, 136, 14, 198, 129, 121, 156,
							 12, 243, 14, 136, 134, 55, 29, 169];
		let mut sk = [0u8; 80];
		sk[..32].copy_from_slice(&pfx);
		sk[32..48].copy_from_slice(&blake2_128(&web3_id[..]));
		sk[48..].copy_from_slice(&web3_id[..]);
		let sv : Option<[u128;5]> = frame_support::storage::unhashed::take(&sk);
		match sv {
			Some(mut a) =>
			{
				frame_support::log::info!("web3 balance before = {:?}", &a[1]);
				a[1] += to_credit;
				frame_support::storage::unhashed::put(&sk,&a);
			},
			None => {
				frame_support::log::info!("web3 balance before = {:?}", 0);
				frame_support::storage::unhashed::put(&sk,&[
					// Necessary after migration. = 2^64
					// It does not affect the balance of course!
					// only the second parameter does.
					18446744073709551616u128,
					to_credit,
					0,
					0,
					0
				]);
			},
		}
		let sv: [u128;5] = frame_support::storage::unhashed::get(&sk).unwrap();
		frame_support::log::info!("web3 balance after = {:?}", &sv[1]);
		0
	}
}

use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};
pub struct AllEdgewareMigrations;
impl OnRuntimeUpgrade for AllEdgewareMigrations {
    fn on_runtime_upgrade() -> Weight {
        let mut weight = 0;

        // #9165 Move PalletVersion away from the crate version
        frame_support::log::info!("💥 MigratePalletVersionToStorageVersion start");
        weight += <MigratePalletVersionToStorageVersion as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 MigratePalletVersionToStorageVersion end");

        // #7040 Fix elections-phragmen and proxy issue
        frame_support::log::info!("💥 PhragmenElectionDepositRuntimeUpgrade start");
        frame_support::traits::StorageVersion::new(0).put::<PhragmenElection>();
        weight += <PhragmenElectionDepositRuntimeUpgrade as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 PhragmenElectionDepositRuntimeUpgrade end");

		// #7930 Allow validators to block and kick their nominator set
		// #8113 Decouple Staking and Election - Part 2.1: Unleash Multi Phase
		// #9507 Implement pallet-bags-list and its interfaces with pallet-staking
		// changes up to v9.
        frame_support::log::info!("💥 AllStakingMigrations start");
        weight += <AllStakingMigrations as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 AllStakingMigrations end");

        // #8221 Self-sufficient account ref-counting.
        frame_support::log::info!("💥 SystemToTripleRefCount start");
        weight += <SystemToTripleRefCount as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 SystemToTripleRefCount end");

		// #8414 Remove Offence delay
        frame_support::log::info!("💥 OffencesDelayCleaningMigration start");
        weight += <OffencesDelayCleaningMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 OffencesDelayCleaningMigration end");

        // #9115 Migrate pallet-collective to the new pallet attribute macro
        frame_support::log::info!("💥 CouncilStoragePrefixMigration start");
        frame_support::traits::StorageVersion::new(0).put::<Council>();
        weight += <CouncilStoragePrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 CouncilStoragePrefixMigration end");

        // #9566 Bounties Pallet to FrameV2
        frame_support::log::info!("💥 BountiesPrefixMigration start");
        frame_support::traits::StorageVersion::new(0).put::<Bounties>();
        weight += <BountiesPrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 BountiesPrefixMigration end");

		// #9711 Migrate pallet-tips to the new pallet attribute macro
        frame_support::log::info!("💥 TipsPalletPrefixMigration start");
        frame_support::traits::StorageVersion::new(0).put::<Tips>();
        weight += <TipsPalletPrefixMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 TipsPalletPrefixMigration end");

		// #10356 Preimage registrar and Scheduler integration
        frame_support::log::info!("💥 SchedulerMigrationV3 start");
        weight += <SchedulerMigrationV3 as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 SchedulerMigrationV3 end");

		// #8231 contracts: Expose rent parameter to contracts
		// #8773 contracts: Move Schedule from Storage to Config
		// #9669 contracts: Remove state rent
		// #10082 contracts: Add storage deposits
		// #10914 contracts: Add test to verify unique trie ids
        frame_support::log::info!("💥 AllContractsMigrations start");
        weight += <AllContractsMigrations as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 AllContractsMigrations end");

		// Web3 foundation specific migration to undo the burning of their inital balance.
        frame_support::log::info!("💥 Web3AccountCreditMigration start");
        weight += <Web3AccountCreditMigration as OnRuntimeUpgrade>::on_runtime_upgrade();
        frame_support::log::info!("😎 Web3AccountCreditMigration end");

        weight
    }
}