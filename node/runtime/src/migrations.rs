// Runtime storage migration file for edgeware

use super::*;
use frame_support::traits::OnRuntimeUpgrade;

pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	(SchedulerMigrationV3,), // append all Migrations here
>;

use super::*;
use frame_support::weights::Weight;
use pallet_indices::Accounts;
use sp_runtime::traits::{One, Zero};

// Migration for scheduler(Preimage) pallet | https://github.com/paritytech/substrate/pull/10356
pub struct SchedulerMigrationV3;
impl OnRuntimeUpgrade for SchedulerMigrationV3 {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		Scheduler::migrate_v2_to_v3()
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

// Phragment election upgrade | pallet-elections-phragmen│   │   ├── pallet-elections-phragmen v5.0.0-dev (https://github.com/paritytech/substrate?branch=polkadot-v0.9.19#174735ea)
//pub struct PhragmenElectionUpgrade;

/* old stuff by Alex:

mod deprecated {
	use crate::Trait;
	use frame_support::{decl_module, decl_storage};
	use sp_std::prelude::*;

	decl_storage! {
		trait Store for Module<T: Trait> as Indices {
			/// The next free enumeration set.
			pub NextEnumSet get(fn next_enum_set): T::AccountIndex;

			/// The enumeration sets.
			pub EnumSet get(fn enum_set): map hasher(opaque_blake2_256) T::AccountIndex => Vec<T::AccountId>;
		}
	}
	decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin { }
	}
}


// Taken from a migration removal PR [here](https://github.com/paritytech/substrate/pull/5870/files#diff-12b2ce0dfddc1915cd81a902d368c2e7L246)
pub fn migrate_enum_set<T: Trait>() -> Weight {
	if deprecated::NextEnumSet::<T>::exists() {
		// migrations need doing.
		let set_count = deprecated::NextEnumSet::<T>::take();
		let set_size: T::AccountIndex = 64.into();

		let mut set_index: T::AccountIndex = Zero::zero();
		while set_index < set_count {
			if !deprecated::EnumSet::<T>::contains_key(&set_index) {
				break;
			}
			let accounts = deprecated::EnumSet::<T>::take(&set_index);
			for (item_index, target) in accounts.into_iter().enumerate() {
				if target != T::AccountId::default() && !T::Currency::total_balance(&target).is_zero() {
					let index = set_index * set_size + T::AccountIndex::from(item_index as u32);
					// We need to add `false` to indicate the index is not frozen.
					// See the [frozen indices PR](https://github.com/paritytech/substrate/pull/6307/)
					Accounts::<T>::insert(index, (target, BalanceOf::<T>::zero(), false));
				}
			}
			set_index += One::one();
		}
		T::MaximumBlockWeight::get()
	} else {
		0
	}
}

*/
