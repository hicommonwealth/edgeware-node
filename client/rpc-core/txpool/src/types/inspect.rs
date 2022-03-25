// Copyright 2019-2021 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

use crate::GetT;
use ethereum::{TransactionAction, TransactionV2 as EthereumTransaction};
use ethereum_types::{H160, H256, U256};
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Summary {
	pub to: Option<H160>,
	pub value: U256,
	pub gas: U256,
	pub gas_price: U256,
}

impl Serialize for Summary {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let res = format!(
			"0x{:x}: {} wei + {} gas x {} wei",
			self.to.unwrap_or_default(),
			self.value,
			self.gas,
			self.gas_price
		);
		serializer.serialize_str(&res)
	}
}

impl GetT for Summary {
	fn get(_hash: H256, _from_address: H160, txn: &EthereumTransaction) -> Self {
	

		Self {
			to: match txn {
				EthereumTransaction::Legacy(tx) => match tx.action {
					TransactionAction::Call(to) => Some(to.as_fixed_bytes().into()),
					_ => None,
				},
				EthereumTransaction::EIP2930(tx) => match tx.action {
					TransactionAction::Call(to) => Some(to.as_fixed_bytes().into()),
					_ => None,
				},
				EthereumTransaction::EIP1559(tx) => match tx.action {
					TransactionAction::Call(to) => Some(to.as_fixed_bytes().into()),
					_ => None,
				},
			},
			value: match txn {
				EthereumTransaction::Legacy(tx) =>  ethereum_types::U256::from(tx.value.into()),
				EthereumTransaction::EIP2930(tx) => ethereum_types::U256::from(tx.value.into()),
				EthereumTransaction::EIP1559(tx) =>  ethereum_types::U256::from(tx.value.into()),
			},
			gas_price: match txn {
				EthereumTransaction::Legacy(tx) =>  ethereum_types::U256::from(tx.gas_price.into()),
				EthereumTransaction::EIP2930(tx) => ethereum_types::U256::from(tx.gas_price.into()),
				EthereumTransaction::EIP1559(tx) =>  ethereum_types::U256::from(tx.max_fee_per_gas.into()),
			},
			gas: match txn {
				EthereumTransaction::Legacy(tx) =>  ethereum_types::U256::from(tx.gas_limit.into()),
				EthereumTransaction::EIP2930(tx) =>  ethereum_types::U256::from(tx.gas_limit.into()),
				EthereumTransaction::EIP1559(tx) =>  ethereum_types::U256::from(tx.gas_limit.into()),
			},
		}
	}
}
