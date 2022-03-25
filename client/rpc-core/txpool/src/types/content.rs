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
use fc_rpc_core::types::Bytes;
use serde::{Serialize, Serializer};

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
	/// Hash
	pub hash: H256,
	/// Nonce
	pub nonce: U256,
	/// Block hash
	#[serde(serialize_with = "block_hash_serialize")]
	pub block_hash: Option<H256>,
	/// Block number
	pub block_number: Option<U256>,
	/// Sender
	pub from: H160,
	/// Recipient
	#[serde(serialize_with = "to_serialize")]
	pub to: Option<H160>,
	/// Transfered value
	pub value: U256,
	/// Gas Price
	pub gas_price: U256,
	/// Gas
	pub gas: U256,
	/// Data
	pub input: Bytes,
	/// Transaction Index
	pub transaction_index: Option<U256>,
}

fn block_hash_serialize<S>(hash: &Option<H256>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_str(&format!("0x{:x}", hash.unwrap_or_default()))
}

fn to_serialize<S>(hash: &Option<H160>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_str(&format!("0x{:x}", hash.unwrap_or_default()))
}


impl GetT for Transaction {
	fn get(hash: H256, from_address: H160, txn: &EthereumTransaction) -> Self {
		
		Self {
			hash,
			nonce: match txn {
				EthereumTransaction::Legacy(tx) => {
					let out: ethereum_types::U256 = ethereum_types::U256::from(tx.nonce.into());
					out
				}
				EthereumTransaction::EIP2930(tx) => ethereum_types::U256::from(tx.nonce.into()),
				EthereumTransaction::EIP1559(tx) =>  ethereum_types::U256::from(tx.nonce.into()),
			},
			block_hash: None,
			block_number: None,
			from: from_address,
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
				EthereumTransaction::EIP2930(tx) =>  ethereum_types::U256::from(tx.value.into()),
				EthereumTransaction::EIP1559(tx) =>  ethereum_types::U256::from(tx.value.into()),
			},
			gas_price: match txn {
				EthereumTransaction::Legacy(tx) =>  ethereum_types::U256::from(tx.gas_price.into()),
				EthereumTransaction::EIP2930(tx) => ethereum_types::U256::from(tx.gas_price.into()),
				EthereumTransaction::EIP1559(tx) => ethereum_types::U256::from(tx.max_fee_per_gas.into()),
			},
			gas: match txn {
				EthereumTransaction::Legacy(tx) =>  ethereum_types::U256::from(tx.gas_limit.into()),
				EthereumTransaction::EIP2930(tx) => ethereum_types::U256::from(tx.gas_limit.into()),
				EthereumTransaction::EIP1559(tx) => ethereum_types::U256::from(tx.gas_limit.into()),
			},
			input: match txn {
				EthereumTransaction::Legacy(tx) => Bytes(tx.input.clone()),
				EthereumTransaction::EIP2930(tx) => Bytes(tx.input.clone()),
				EthereumTransaction::EIP1559(tx) => Bytes(tx.input.clone()),
			},
			transaction_index: None,
		}
	}
}
