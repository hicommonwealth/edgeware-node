// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! http://research.web3.foundation/en/latest/polkadot/Token%20Economics/#inflation-model

use sr_primitives::{Perbill, traits::SimpleArithmetic};

/// Linear function truncated to positive part `y = max(0, b [+ or -] a*x)` for PNPoS usage
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Linear {
	negative_a: bool,
	// Perbill
	a: u32,
	// Perbill
	b: u32,
}

impl Linear {
	fn calculate_for_fraction_times_denominator<N>(&self, n: N, d: N) -> N
	where
		N: SimpleArithmetic + Clone
	{
		if self.negative_a {
			(Perbill::from_parts(self.b) * d).saturating_sub(Perbill::from_parts(self.a) * n)
		} else {
			(Perbill::from_parts(self.b) * d).saturating_add(Perbill::from_parts(self.a) * n)
		}
	}
}

/// Piecewise Linear function for PNPoS usage
#[derive(Debug, PartialEq, Eq)]
struct PiecewiseLinear {
	/// Array of tuple of Abscisse in Perbill and Linear.
	///
	/// Each piece start with at the abscisse up to the abscisse of the next piece.
	pieces: [(u32, Linear); 20],
}

impl PiecewiseLinear {
	fn calculate_for_fraction_times_denominator<N>(&self, n: N, d: N) -> N
	where
		N: SimpleArithmetic + Clone
	{
		let part = self.pieces.iter()
			.take_while(|(abscisse, _)| n > Perbill::from_parts(*abscisse) * d.clone())
			.last()
			.unwrap_or(&self.pieces[0]);

		part.1.calculate_for_fraction_times_denominator(n, d)
	}
}

// Piecewise linear approximation of I_NPoS.
const I_NPOS: PiecewiseLinear = PiecewiseLinear {
	pieces: [
		(0, Linear { negative_a: false, a: 150000000, b: 25000000 }),
		(500000000, Linear { negative_a: true, a: 986493987, b: 593246993 }),
		(507648979, Linear { negative_a: true, a: 884661327, b: 541551747 }),
		(515726279, Linear { negative_a: true, a: 788373842, b: 491893761 }),
		(524282719, Linear { negative_a: true, a: 697631517, b: 444319128 }),
		(533378749, Linear { negative_a: true, a: 612434341, b: 398876765 }),
		(543087019, Linear { negative_a: true, a: 532782338, b: 355618796 }),
		(553495919, Linear { negative_a: true, a: 458675508, b: 314600968 }),
		(564714479, Linear { negative_a: true, a: 390113843, b: 275883203 }),
		(576879339, Linear { negative_a: true, a: 327097341, b: 239530285 }),
		(590164929, Linear { negative_a: true, a: 269626004, b: 205612717 }),
		(604798839, Linear { negative_a: true, a: 217699848, b: 174207838 }),
		(621085859, Linear { negative_a: true, a: 171318873, b: 145401271 }),
		(639447429, Linear { negative_a: true, a: 130483080, b: 119288928 }),
		(660489879, Linear { negative_a: true, a: 95192479, b: 95979842 }),
		(685131379, Linear { negative_a: true, a: 65447076, b: 75600334 }),
		(714860569, Linear { negative_a: true, a: 41246910, b: 58300589 }),
		(752334749, Linear { negative_a: true, a: 22592084, b: 44265915 }),
		(803047659, Linear { negative_a: true, a: 9482996, b: 33738693 }),
		(881691659, Linear { negative_a: true, a: 2572702, b: 27645944 })
	]
};

/// Second per year for the Julian year (365.25 days)
const SECOND_PER_YEAR: u32 = 3600*24*36525/100;

/// The total payout to all validators (and their nominators) per era.
///
/// Named P_NPoS in the [paper](http://research.web3.foundation/en/latest/polkadot/Token%20Ec
/// onomics/#inflation-model).
///
/// For x the staking rate in NPoS: `P_NPoS(x) = I_NPoS(x) * current_total_token / era_per_year`
/// i.e.  `P_NPoS(x) = I_NPoS(x) * current_total_token * era_duration / year_duration`
///
/// I_NPoS is the desired yearly inflation rate for nominated proof of stake.
pub fn compute_total_payout<N>(npos_token_staked: N, total_tokens: N, era_duration: N) -> N
where
	N: SimpleArithmetic + Clone
{
	let year_duration: N = SECOND_PER_YEAR.into();
	I_NPOS.calculate_for_fraction_times_denominator(npos_token_staked, total_tokens)
		* era_duration / year_duration
}
