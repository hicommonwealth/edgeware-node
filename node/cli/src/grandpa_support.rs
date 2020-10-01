// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot-specific GRANDPA integration utilities.

use edgeware_primitives::Hash;
use sp_runtime::traits::{Block as BlockT, NumberFor};

/// A custom GRANDPA voting rule that "pauses" voting (i.e. keeps voting for the
/// same last finalized block) after a given block at height `N` has been
/// finalized and for a delay of `M` blocks, i.e. until the best block reaches
/// `N` + `M`, the voter will keep voting for block `N`.
pub(crate) struct PauseAfterBlockFor<N>(pub(crate) N, pub(crate) N);

impl<Block, B> sc_finality_grandpa::VotingRule<Block, B> for PauseAfterBlockFor<NumberFor<Block>> where
	Block: BlockT,
	B: sp_blockchain::HeaderBackend<Block>,
{
	fn restrict_vote(
		&self,
		backend: &B,
		base: &Block::Header,
		best_target: &Block::Header,
		current_target: &Block::Header,
	) -> Option<(Block::Hash, NumberFor<Block>)> {
		use sp_runtime::generic::BlockId;
		use sp_runtime::traits::Header as _;

		// walk backwards until we find the target block
		let find_target = |
			target_number: NumberFor<Block>,
			current_header: &Block::Header
		| {
			let mut target_hash = current_header.hash();
			let mut target_header = current_header.clone();

			loop {
				if *target_header.number() < target_number {
					unreachable!(
						"we are traversing backwards from a known block; \
						 blocks are stored contiguously; \
						 qed"
					);
				}

				if *target_header.number() == target_number {
					return Some((target_hash, target_number));
				}

				target_hash = *target_header.parent_hash();
				target_header = backend.header(BlockId::Hash(target_hash)).ok()?
					.expect("Header known to exist due to the existence of one of its descendents; qed");
			}
		};

		// only restrict votes targeting a block higher than the block
		// we've set for the pause
		if *current_target.number() > self.0 {
			// if we're past the pause period (i.e. `self.0 + self.1`)
			// then we no longer need to restrict any votes
			if *best_target.number() > self.0 + self.1 {
				return None;
			}

			// if we've finalized the pause block, just keep returning it
			// until best number increases enough to pass the condition above
			if *base.number() >= self.0 {
				return Some((base.hash(), *base.number()));
			}

			// otherwise find the target header at the pause block
			// to vote on
			return find_target(self.0, current_target);
		}

		None
	}
}

/// GRANDPA hard forks due to borked migration of session keys after a runtime
/// upgrade (at #1491596), the signalled authority set changes were invalid
/// (blank keys) and were impossible to finalize. The authorities for these
/// intermediary pending changes are replaced with a static list comprised of
/// w3f validators and randomly selected validators from the latest session (at
/// #1500988).
pub(crate) fn edgeware_hard_forks() -> Vec<(
	sp_finality_grandpa::SetId,
	(Hash, edgeware_primitives::BlockNumber),
	sp_finality_grandpa::AuthorityList,
)> {
	use sp_core::crypto::Ss58Codec;
	use std::str::FromStr;
	let forks = vec![];
	// populate with Edgeware GRANDPA authority set
	let authorities = vec![];

	let authorities = authorities
		.into_iter()
		.map(|address| {
			(
				sp_finality_grandpa::AuthorityId::from_ss58check(address)
					.expect("hard fork authority addresses are static and they should be carefully defined; qed."),
				1,
			)
		})
		.collect::<Vec<_>>();

	forks
		.into_iter()
		.map(|(set_id, hash, number)| {
			let hash = Hash::from_str(hash)
				.expect("hard fork hashes are static and they should be carefully defined; qed.");

			(set_id, (hash, number), authorities.clone())
		})
		.collect()
}
