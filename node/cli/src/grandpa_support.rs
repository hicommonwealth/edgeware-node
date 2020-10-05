// Copyright 2018-2020 Commonwealth Labs, Inc.
// This file is part of Edgeware.

// Edgeware is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Edgeware is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>.

//! Edgeware-specific GRANDPA integration utilities.


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
	let authorities = vec![
		"npBafMuLgBtYKjLUn1ttEa6fQF5jfNNroMVaHUu1PEkQaE7",
		"ieUUdxeM4WnY4kyr58kmn2ouNVZCkpmJfhNihuKTLoXLG4n",
		"mN9tTMjQ1ipQN9n4qbdugBNfaet2eUbPUgf13NEPZswHxpQ",
		"hb5HDDbrjcuKmCqj6a76Np6dH64ChGnznTFPrPADwaNmVX4",
		"nC8FX4rLyyTHBbp1f5in6pjEGSzDnMVisrksJVySwi9YbEQ",
		"j76GLp9gdrj96dHtUYkosx4y6DL4pEWxaEbQ549f8S7WzU2",
		"ihuTJSMSyPhoHhg4UnBQqyQL1x8xVP1w4s32h1Tytm86NvM",
		"iJ2jocfuhcDs38WDQjT3gfT1zx8dvXFQmNndsEtaTVBsnXm",
		"kKu3v8ZdJA9xCdwBNgxpEPgKWKPduxZ7jq1AtUaNqQv15aK",
		"jtGYmEBYLSDoHA3pqYVFvAp2hwNHNYZt34kTW6FovuiD8iD",
		"jT18C2DpEiXhNz7MX8iAQNzkAXRuh2zeeoktK5yidW1t4SU",
		"mtGH9K3sQgUyGEBH6ntPWJ7ADAqLJQo35Z8afzsXe6N5bsg",
		"hy11tm4AfYsyCZwNpXhZDBnPBtZrxY6N63V5Z1uBFTUqsLp",
		"jsLtGDm5NWvGhYb29ejbJACG6ecDdDwn6qnZYXT1Z1ddeoD",
		"jpyus9SFT9i5yyYhuey2uwRNas48YuHPqEZBv9XTCx6rF3J",
		"jh3xaLPVTQreqpBpsaAGGBPWF2EE1h2r1nD4637adAEx3iJ",
		"idtQsQmfeJMQm6spdU3HpWmQtG3pKpxNnLH6StxujHVCLZp",
		"oA9LoLjDiCRAfaKz45ZYfd4hQ7KbteCYjUqjxu751bKiypn",
		"mCswpvyiM4KzGt8mzrjKch8QsHVjepjTHg8cdv9bcyfn5JW",
		"njKY54vkH5e6NHZfhydETsCC4C9pSr1R5NZJsWmNxAdUkjk",
		"k9TnYmf1Dkf24MfPUf1X6hQPgv31EFQLp8rVVkGw6ucXeqE",
		"iWJ1utx5LP6MtRhspQADtXFPjF9RVxetkTMkWnkqoXpR3Yf",
		"jfmMbQoVEGFfVBGXQhELhkQWZXtAsZCofWEoTsehkqGSy91",
		"iNLnboDZvM3wpeGFhjpmrnmV1tMVpjCLLmv7Exry1ZL1kfb",
		"numkc8nBXjVT1j2kUcdVDJuLJmZrjEcthHnEKXBMRf8vQzz",
		"iK42YwgHkVsgUNPa5hziVTZGMiFgmockcUNvT2zFeazh3oR",
		"iCt7pCkLD6Jht2R4g8GKTxprxAnHh5NFc3EsKpzMnpu4VRV",
		"oAeRZiVgQMePJ9X53AfqdV1bPoj71JapHwKx6eEGqjtmd1J",
		"o8yuL8pEHcvmgfG3JNBuAPnRqmzEnRrhZaNAm1NBYFddqaB",
		"hxTJDSsN1WDiRsYweD4qZwE8wofG7TXCc3scvRez7s2nGgu",
		"i38gersgmoFud4hMAg4MTytqpjt6jvRRFnaM8vYBs3Akt1Y",
		"meBCH11eSmLCACbLKND7aZbaQdQwd59dp8iLpCzMC8E8Jkc",
		"jctWxARSKAgcQ6aBZ5VxetcSgMKydCrfrMYZVDrKSGh23RY",
		"jbzqLXGBdNZSQPzWqKD2asAcH9cCVbNUXsioqBSgX1cjXfC",
		"kVURsdTpfNwiNRotsZkYpf7DSzumEXaJjDN9JPQcoMa2EbU",
		"ky2HskSqdzmfhtAD2222r8QutG4uAjuUWQnbUPUziR1uSde",
		"kHCKF9KhSdp6DyUtXXfLhrqJwYRm8N6cL2esG22PjQHkPZL",
		"jP62bakXLtxfHG31TL9CXn7aNFfrekxUYEwxHjebdPGFCUw",
		"mBaaUTdegweQ1qZeoP4p7254K3eCWL3ZyEKdsJd6oyCFg25",
		"jFjPE4m7ypDav1nPxuS4zmfiPwfqpVygZBWsJrnGgE9nUTn",
		"kvuHosMVD7VPodPHQ6BXiHDy94frvtbj6qGRGS1ibpzSWDT",
		"kUQA8gvP3eG5aVEycz5puMqsTxiAdJZcaSArUuvKpvQiMJA",
		"kCPoSCwz2ws58JA4YwePd637orMvSdx96kmupt4hxi3wfoW",
		"hpu9ogcR5AW894eS6Zq5uUZpUWyqMftMWPbc3c26qkafiz9",
		"jnRsZWQeesyXeLV8brEKRkQKTR9HdJM2x4oqaS7g1GZjVqx",
		"kbLoxRS48VTshSziV82cThLmEjHMYVcKALWcSvL9SuC37X8",
		"mXHo4jHUKkVJiwhP6X68XZ41JdbzXLvsDwPb74nd9KdUvrJ",
		"mbu9KgYsVKVTsNDhSVMRZ4PxZhTqLUoPueN9DSiGi8pZ2Bu",
		"kTbLehtNqU3CciU11b79miaM3f4cLm6B4AAJo7isBpV8FQ2",
		"nu4CQJsvyz41WgnuQ76LB3gFt7nRNrx1fjPe31jwyEHAWNh",
		"iGujtF17kdfVpHV5MqZ2j4mRhwQVvuyJBva6d97SiBtcDKT",
		"iT2Uvp5fd6HvvDnnS4g3qtAc6iycuscLVrQ9K7arnifJXsZ",
		"m1xtFgg5akWWwegXdfGbWb4Au5JfgxUJmuKkcGgXRF4PhQW",
		"kJQZmFwcVUdTep6dAPBzPfUZUodvgVpmHrAYkNSsEWHBwfT",
		"koCfRPkrUmHnPWSEixaE9MhKoeZLNuxcqoJjemR2QEy29Ht",
		"jicPBHhLUeYBeEwnUAkxSEWqK2bvinxrG24EQW2rZ6tbSxc",
		"nHUa3CZALignw1GyPNd1GCKFNB8oRGvKUTpnyX1xhijhW6v",
		"hsoTJKThfEAM4VrBnt3AN9n3Qx7HZ8kD33xzaxkmaRa5paL",
		"jKhPvN1okzTwo55mdaeRXkCPYuwRudocoirthPGXEp23BFw",
		"kiZyft4pzS5EaLCCvsKxXLKkq9yZHh4RD4owX6LxXJjrYQT",
		"id9UBMLHXuzZGos2SPSwbdoQJxFthcid9mWwzPWyNZUqtdn",
		"kdQ7SkTSc8X3pDKfqKpfbNFkevA3RVB7Mtpi6umk49GHVEK",
		"ktntoG1i9bM2N8tMuAbETreD6hH7a3Jua6mzWSmrP1Jr65K",
		"hsmeh7XUSKDWc3Bg8AugjACMhN8dv6KYfVRVwBUaZuMZs46",
		"mKYxzWGtCSs4HC9RswKA1URs5UXk19zQh4BSemmoLXyWBgz",
		"iMJMxsR2XdCZXyi6Bmnbq2zW9iEqDo7rMfSM6VvrUiWj69z",
		"hs6vujefB2N35jXrrTwyfNYknArgdAPh2DRjX4zsHJrTi9W",
		"jnQdVLj7DvHqbgL6dY7PdAdfBs71X77J879HEJ1Qmirtvq6",
		"jjf3MNiJzr1iAh4z96A5xvkikgpDZj5G2YZRygHSV6jxHy1",
		"naJ8sywAhYLU9A8MtAZF71GpmRbm6SbkrkghzM5PVKzj1pc",
		"imK7y8RydtdHFu5VxZwU7Se87qZ21uRkDAoSTKWdddCuUJV",
		"ivxivcTJ2J1WNTEv15nwgDV3EPvJcyp1PwJpJVXfaqR2QPo",
		"hu496tZQgKca6sYKhAm4Sz5bSBV5FG9CZZFev68peELrWqF",
		"kUyFmgxo5f8Z8Fxh1QpK7ZndcC82tp32Nj25LDnaggiFRT5",
		"kECXJxPQRi3GN9gKuv8iDQNXxkk8CYsXBJKzqBPFLJ6Zsrn",
		"jCmbVGHTW2uhtcV1JwhiokAAX5gnNYD67EzbtPjZtRvcmoz",
		"j2yiDbNgz6kHsD85a3GDbP37WQrRa33tisS9Qur2HN2b7bL",
		"k5zYN4LTCwQifeq7r88kKLtiDZijxVCpmDEvyJ9oceYNtZE",
		"ivsaVjDoWm5vciw13btb2ePZs8DDgd1iqnfvcL4ZZmvJ9Ey",
		"o9Lt7UVHTDDHZy1dujM3wt4vude8cQ97BC7Rh5nKecY6G5L",
		"o1oTZj582Q6aVq4AS27t57dNqKLv5i1a9xy2uqLi1dfguyJ",
		"iHxx49W1W5tBfSMJDnRySEpLQzm2DAPHUcEsSxkVZACJMXc",
		"mnTtx6db9tcEKYdwBeuiB7YYRFDyCWt9TRtwQSWrDXu866g",
		"kwz4jjWvBTZVvwsKLsBbSqCshGbRopNbV3rEzJb8i9chfWf",
		"i3amHWhsemEWgPYkzWiATbKGNLF4mZ1kkDpXVVbAjjCXwUk",
		"oFBMiGKEzBTTbh1Nxw1GfDfksJ4fXZRNeLi7VvFZzVtpCcz",
		"nMiipdFd3SfN2Z9SwpQN4Dwqj46m4tjz2eitgBBc3BAau5i",
		"mXbGPeSbbejxECrykiMP3rZQzhfg1JEbfSk4pgLoYhYQTwv",
		"hc2pubgRorM662QijiK3SFjqGWs16zzXcAmEWF7oGPAPFx1",
		"iJu9hHNKFrFMfGA4ZZNhymFCnQGAjyNiqwa9Ynb3wSjKvV4",
		"ktZWfLz8HbZYXyFqKvkTCNVqYpoCS632ySStZjUanqFQrc4",
		"nX22FtaihpzVxsxmqGw4oJku7Q9ASixQBAj76tobC4EN3DL",
		"kBt24MFJHDLfpGBM4dhS3BojRSza6exgyLvyXWt7YFx9Q8D",
		"iPFGXCoPPFJXNjU4bZepVDnMAP8JADN6bXNjTwjNAwuJ2t9",
		"jxKPRYx8JKacvfvNBrowFyczhMppxgDvCnVQPL5MMAyvNch",
	];

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

#[cfg(test)]
mod tests {
	use edgeware_test_runtime_client::prelude::*;
	use edgeware_test_runtime_client::sp_consensus::BlockOrigin;
	use sc_block_builder::BlockBuilderProvider;
	use sc_finality_grandpa::VotingRule;
	use sp_blockchain::HeaderBackend;
	use sp_runtime::generic::BlockId;
	use sp_runtime::traits::Header;
	use std::sync::Arc;

	#[test]
	fn grandpa_pause_voting_rule_works() {
		let client = Arc::new(edgeware_test_runtime_client::new());

		let mut push_blocks = {
			let mut client = client.clone();
			move |n| {
				for _ in 0..n {
					let b = client.new_block(Default::default()).unwrap().build();
					let block = b.unwrap().block;
					client.import(BlockOrigin::Own, block).unwrap();
				}
			}
		};

		let get_header = {
			let client = client.clone();
			move |n| client.header(&BlockId::Number(n)).unwrap().unwrap()
		};

		// the rule should filter all votes after block #20
		// is finalized until block #50 is imported.
		let voting_rule = super::PauseAfterBlockFor(20, 30);

		// add 10 blocks
		push_blocks(10);
		assert_eq!(
			client.info().best_number,
			10,
		);

		// we have not reached the pause block
		// therefore nothing should be restricted
		assert_eq!(
			voting_rule.restrict_vote(
				&*client,
				&get_header(0),
				&get_header(10),
				&get_header(10),
			),
			None,
		);

		// add 15 more blocks
		// best block: #25
		push_blocks(15);

		// we are targeting the pause block,
		// the vote should not be restricted
		assert_eq!(
			voting_rule.restrict_vote(
				&*client,
				&get_header(10),
				&get_header(20),
				&get_header(20),
			),
			None,
		);

		// we are past the pause block, votes should
		// be limited to the pause block.
		let pause_block = get_header(20);
		assert_eq!(
			voting_rule.restrict_vote(
				&*client,
				&get_header(10),
				&get_header(21),
				&get_header(21),
			),
			Some((pause_block.hash(), *pause_block.number())),
		);

		// we've finalized the pause block, so we'll keep
		// restricting our votes to it.
		assert_eq!(
			voting_rule.restrict_vote(
				&*client,
				&pause_block, // #20
				&get_header(21),
				&get_header(21),
			),
			Some((pause_block.hash(), *pause_block.number())),
		);

		// add 30 more blocks
		// best block: #55
		push_blocks(30);

		// we're at the last block of the pause, this block
		// should still be considered in the pause period
		assert_eq!(
			voting_rule.restrict_vote(
				&*client,
				&pause_block, // #20
				&get_header(50),
				&get_header(50),
			),
			Some((pause_block.hash(), *pause_block.number())),
		);

		// we're past the pause period, no votes should be filtered
		assert_eq!(
			voting_rule.restrict_vote(
				&*client,
				&pause_block, // #20
				&get_header(51),
				&get_header(51),
			),
			None,
		);
	}
}