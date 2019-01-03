// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Personal rpc interface.
use eip712::EIP712;
use jsonrpc_core::types::Value;
use jsonrpc_core::{BoxFuture, Result};
use v1::types::{Bytes, U128, H160, H256, H520, TransactionRequest, RichRawTransaction as RpcRichRawTransaction, EIP191Version};

build_rpc_trait! {
	pub trait Clique {
		type Metadata;

		/// Lists all stored accounts
		#[rpc(name = "personal_listAccounts")]
		fn propose(&self, address: H160, bool) -> Result<Vec<H160>>;
}
