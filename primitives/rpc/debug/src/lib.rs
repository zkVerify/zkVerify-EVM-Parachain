// Copyright 2025, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use core::result::Result;

use ethereum::TransactionV2 as Transaction;
use ethereum_types::H256;
use parity_scale_codec::{Decode, Encode};
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {

    // We realized that even using runtime overrides, using the ApiExt interface reads the api
    // versions from the state runtime, meaning we cannot just reset the versioning as we see fit.
    //
    // In order to be able to use ApiExt as part of the RPC handler logic we need to be always
    // above the version that exists on chain for this Api, even if this Api is only meant
    // to be used overridden.
    pub trait DebugRuntimeApi {

        fn trace_transaction(
            extrinsics: Vec<Block::Extrinsic>,
            transaction: &Transaction,
        ) -> Result<(), sp_runtime::DispatchError>;

        fn trace_block(
            extrinsics: Vec<Block::Extrinsic>,
            known_transactions: Vec<H256>,
        ) -> Result<(), sp_runtime::DispatchError>;
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Encode, Decode)]
pub enum TracerInput {
    None,
    Blockscout,
    CallTracer,
}

/// DebugRuntimeApi V2 result. Trace response is stored in client and runtime api call response is
/// empty.
#[derive(Debug)]
pub enum Response {
    Single,
    Block,
}
