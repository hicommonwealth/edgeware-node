use democracy_current::*;
use democracy_deprecated as deprecated;

pub fn migrate_account<T: Trait>(a: &T::AccountId) {
	Locks::<T>::migrate_key_from_blake(a);
}

// The edgeware migration is so big we just assume it consumes the whole block.
pub fn migrate_all<T: Trait>() -> Weight {
	sp_runtime::print("🕊️  Migrating Democracy...");
	let mut weight = T::MaximumBlockWeight::get();
	sp_runtime::print("Democracy: Hasher");
	weight += migrate_hasher::<T>();
	sp_runtime::print("Democracy: Remove Unused");
	weight += migrate_remove_unused_storage::<T>();
	sp_runtime::print("Democracy: ReferendumInfo");
	weight += migrate_referendum_info::<T>();
	sp_runtime::print("🕊️  Done Democracy.");
	weight
}

pub fn migrate_hasher<T: Trait>() -> Weight {
	// Edgeware does not have any blacklist/cancellations that need to be migrated --> remove
	Blacklist::<T>::remove_all();
	Cancellations::<T>::remove_all();
	// Note this only migrates the hasher, `ReferendumInfoOf` is fully migrated in
	// `migrate_referendum_info`.
	sp_runtime::print("Democracy: Hasher: ReferendumInfo");
	for i in LowestUnbaked::get()..ReferendumCount::get() {
		deprecated::ReferendumInfoOf::<T>::migrate_key_from_blake(i);
	}
	sp_runtime::print("Democracy: Hasher: PublicProps");
	for (prop_idx, prop_hash, _) in PublicProps::<T>::get().into_iter() {
		// based on [democracy weights PR](https://github.com/paritytech/substrate/pull/5828/)
		if let Some((deposit, depositors)) = deprecated::DepositOf::<T>::take(prop_idx) {
			DepositOf::<T>::insert(prop_idx, (depositors, deposit));
		}
		// necessary because of [scheduler PR](https://github.com/paritytech/substrate/pull/5412)
		if let Some((data, provider, deposit, since)) = deprecated::Preimages::<T>::take(prop_hash) {
			Preimages::<T>::insert(prop_hash, PreimageStatus::Available{data, provider, deposit, since, expiry: None});
		}
	}
	0
}

pub fn migrate_remove_unused_storage<T: Trait>() -> Weight {
	// It's unlikely that Edgeware will have open proposals during the migration so we can assume
	// this to be fine.
	deprecated::VotersFor::<T>::remove_all();
	deprecated::VoteOf::<T>::remove_all();
	deprecated::Proxy::<T>::remove_all();
	deprecated::Delegations::<T>::remove_all();
	0
}

// migration based on [substrate/#5294](https://github.com/paritytech/substrate/pull/5294)
pub fn migrate_referendum_info<T: Trait>() -> Weight {
	use frame_support::{Twox64Concat, migration::{StorageKeyIterator}};
	
	let range = LowestUnbaked::get()..ReferendumCount::get();
	for (index, deprecated::ReferendumInfo { end, proposal_hash, threshold, delay})
		in StorageKeyIterator::<
			ReferendumIndex,
			deprecated::ReferendumInfo,
			Twox64Concat,
		>::new(b"Democracy", b"ReferendumInfoOf").drain()
	{
		if range.contains(&index) {
			let status = ReferendumStatus {
				end, proposal_hash, threshold, delay, tally: Tally::default()
			};
			ReferendumInfoOf::<T>::insert(index, ReferendumInfo::Ongoing(status))
		}
	}
	0
}