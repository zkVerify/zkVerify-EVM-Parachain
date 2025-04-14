// Copyright 2025, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use frame_support::{parameter_types, traits::AsEnsureOriginWithArg};
use frame_system::{EnsureRoot, EnsureSigned};
use parity_scale_codec::{Compact, Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::Hash as THash;
use sp_std::{convert::From, prelude::*};
use xcm::latest::Location;

use crate::{
    constants::currency::{deposit, CENTS, MILLICENTS},
    types::{AccountId, AssetId, Balance},
    weights, Balances, Runtime, RuntimeEvent,
};

parameter_types! {
    pub const AssetDeposit: Balance = 10 * CENTS;
    pub const AssetAccountDeposit: Balance = deposit(1, 16);
    pub const ApprovalDeposit: Balance = MILLICENTS;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = deposit(1, 68);
    pub const MetadataDepositPerByte: Balance = deposit(0, 1);
    pub const RemoveItemsLimit: u32 = 1000;
}

// Required for runtime benchmarks
pallet_assets::runtime_benchmarks_enabled! {
    pub struct BenchmarkHelper;
    impl<AssetIdParameter> pallet_assets::BenchmarkHelper<AssetIdParameter> for BenchmarkHelper
    where
        AssetIdParameter: From<u32>,
    {
        fn create_asset_id_parameter(id: u32) -> AssetIdParameter {
            id.into()
        }
    }
}

// Foreign assets
impl pallet_assets::Config for Runtime {
    type ApprovalDeposit = ApprovalDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type AssetDeposit = AssetDeposit;
    type AssetId = AssetId;
    type AssetIdParameter = Compact<AssetId>;
    type Balance = Balance;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = BenchmarkHelper;
    type CallbackHandle = ();
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
    type Currency = Balances;
    type Extra = ();
    type ForceOrigin = EnsureRoot<AccountId>;
    type Freezer = ();
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type RemoveItemsLimit = RemoveItemsLimit;
    type RuntimeEvent = RuntimeEvent;
    type StringLimit = StringLimit;
    type WeightInfo = weights::pallet_assets::ZKVEvmWeight<Runtime>; //FIXME: run & update
}

// Our AssetType. For now we only handle Xcm Assets
#[derive(Clone, Eq, Debug, PartialEq, Ord, PartialOrd, Encode, Decode, TypeInfo)]
pub enum AssetType {
    Xcm(xcm::v3::Location),
}
impl Default for AssetType {
    fn default() -> Self {
        Self::Xcm(xcm::v3::Location::here())
    }
}

impl From<xcm::v3::Location> for AssetType {
    fn from(location: xcm::v3::Location) -> Self {
        Self::Xcm(location)
    }
}

// This can be removed once we fully adopt xcm::v4 everywhere
impl TryFrom<Location> for AssetType {
    type Error = ();

    fn try_from(location: Location) -> Result<Self, Self::Error> {
        Ok(Self::Xcm(location.try_into()?))
    }
}

impl From<AssetType> for Option<xcm::v3::Location> {
    fn from(val: AssetType) -> Self {
        match val {
            AssetType::Xcm(location) => Some(location),
        }
    }
}

// Implementation on how to retrieve the AssetId from an AssetType
impl From<AssetType> for AssetId {
    fn from(asset: AssetType) -> AssetId {
        match asset {
            AssetType::Xcm(id) => {
                let mut result: [u8; 4] = [0u8; 4];
                let hash: H256 = id.using_encoded(<Runtime as frame_system::Config>::Hashing::hash);
                result.copy_from_slice(&hash.as_fixed_bytes()[0..4]);
                u32::from_le_bytes(result)
            }
        }
    }
}
