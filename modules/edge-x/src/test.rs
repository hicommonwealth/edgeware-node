#[cfg(test)]
mod tests {
	use super::*;

	use support::{assert_ok, impl_outer_origin, parameter_types, weights::GetDispatchInfo};
	use primitives::H256;
	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
	use sr_primitives::{
		Perbill, testing::Header,
		traits::{BlakeTwo256, OnInitialize, OnFinalize, IdentityLookup},
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: u32 = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Call = ();
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}
	parameter_types! {
		pub const ExistentialDeposit: u64 = 0;
		pub const TransferFee: u64 = 0;
		pub const CreationFee: u64 = 0;
	}
	impl balances::Trait for Test {
		type Balance = u64;
		type OnFreeBalanceZero = ();
		type OnNewAccount = ();
		type Event = ();
		type TransferPayment = ();
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type TransferFee = TransferFee;
		type CreationFee = CreationFee;
	}
	impl Trait for Test {
		type Event = ();
	}
	type Example = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities {
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		// We use default for brevity, but you can configure as desired if needed.
		balances::GenesisConfig::<Test>::default().assimilate_storage(&mut t).unwrap();
		GenesisConfig::<Test>{
			dummy: 42,
			// we configure the map with (key, value) pairs.
			bar: vec![(1, 2), (2, 3)],
			foo: 24,
		}.assimilate_storage(&mut t).unwrap();
		t.into()
	}

	#[test]
	fn it_works_for_optional_value() {
		new_test_ext().execute_with(|| {
			// Check that GenesisBuilder works properly.
			assert_eq!(Example::dummy(), Some(42));

			// Check that accumulate works when we have Some value in Dummy already.
			assert_ok!(Example::accumulate_dummy(Origin::signed(1), 27));
			assert_eq!(Example::dummy(), Some(69));

			// Check that finalizing the block removes Dummy from storage.
			<Example as OnFinalize<u64>>::on_finalize(1);
			assert_eq!(Example::dummy(), None);

			// Check that accumulate works when we Dummy has None in it.
			<Example as OnInitialize<u64>>::on_initialize(2);
			assert_ok!(Example::accumulate_dummy(Origin::signed(1), 42));
			assert_eq!(Example::dummy(), Some(42));
		});
	}

	#[test]
	fn it_works_for_default_value() {
		new_test_ext().execute_with(|| {
			assert_eq!(Example::foo(), 24);
			assert_ok!(Example::accumulate_foo(Origin::signed(1), 1));
			assert_eq!(Example::foo(), 25);
		});
	}

	#[test]
	fn signed_ext_watch_dummy_works() {
		new_test_ext().execute_with(|| {
			let call = <Call<Test>>::set_dummy(10);
			let info = DispatchInfo::default();

			assert_eq!(
				WatchDummy::<Test>(PhantomData).validate(&1, &call, info, 150)
					.unwrap()
					.priority,
				Bounded::max_value(),
			);
			assert_eq!(
				WatchDummy::<Test>(PhantomData).validate(&1, &call, info, 250),
				InvalidTransaction::ExhaustsResources.into(),
			);
		})
	}

	#[test]
	fn weights_work() {
		// must have a default weight.
		let default_call = <Call<Test>>::accumulate_dummy(10);
		let info = default_call.get_dispatch_info();
		// aka. `let info = <Call<Test> as GetDispatchInfo>::get_dispatch_info(&default_call);`
		assert_eq!(info.weight, 10_000);

		// must have a custom weight of `100 * arg = 2000`
		let custom_call = <Call<Test>>::set_dummy(20);
		let info = custom_call.get_dispatch_info();
		assert_eq!(info.weight, 2000);
	}
}