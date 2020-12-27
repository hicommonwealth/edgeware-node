use super::*;

use frame_benchmarking::{benchmarks, account, whitelist_account};
use frame_system::{RawOrigin, self};


const SEED: u32 = 0;


benchmarks! {
	_ { }

	add_ren_token {
		// let p in 1 .. MAX_PROPOSALS;
		// let b in 2 .. MAX_BYTES;
		//
		// let proposer = funded_account::<T>("proposer", 0);
		// whitelist_account!(proposer);
		// let origin: <T as frame_system::Config>::Origin = RawOrigin::Signed(proposer.clone()).into();
		//
		// let title: &[u8] = b"Edgeware";
		//
		// // Create p existing proposals
		// for i in 0 .. p {
		// 	let contents = vec![i.to_le_bytes()[0]; b as usize];
		// 	let outcomes = vec![YES_VOTE, NO_VOTE];
		// 	Signaling::<T>::create_proposal(origin.clone(), title.into(), contents, outcomes, VoteType::Binary, TallyType::OneCoin, VotingScheme::Simple)?;
		// }
		// assert_eq!(Signaling::<T>::inactive_proposals().len(), p as usize);
		//
		// // create new proposal
		// let contents = vec![p.to_le_bytes()[0]; b as usize];
		// let outcomes = vec![YES_VOTE, NO_VOTE];
		//
		// let mut buf = Vec::new();
		// buf.extend_from_slice(&proposer.encode());
		// buf.extend_from_slice(&contents.as_ref());
		// let hash = T::Hashing::hash(&buf[..]);
	}: _(RawOrigin::Root,Default::default(),Default::default(),"renBTC".into(),[0u8; 32],[0u8; 20],true,true,0u32.into(),0u32.into()) //RawOrigin::Signed(proposer.clone()), title.into(), contents, outcomes, VoteType::Binary, TallyType::OneCoin, VotingScheme::Simple)
	verify {
		// assert!(Signaling::<T>::proposal_of(hash).is_some());
		// assert_eq!(Signaling::<T>::inactive_proposals().len(), (p+1) as usize);
	}

}

#[cfg(test)]
mod tests {
	use super::*;
	use mock::{Runtime, ExtBuilder, AccountId, Balances, Origin, RenVmBridge, RenvmBridgeCall, System, Assets};
	use frame_support::assert_ok;

	fn test_benchmarks() {
		ExtBuilder::default().build().execute_with(|| {
			assert_ok!(test_benchmark_add_ren_token::<Runtime>());
		});
	}
}
