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

use crate::{configs::xcm::*, Runtime, RuntimeOrigin};
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_core::{H256, U256};
use sp_std::{boxed::Box, marker::PhantomData, vec};
use xcm::v5::{Asset, Assets, Fungibility, Junction, Location};
use xcm::{VersionedAssets, VersionedLocation};

pub struct XcmTeleportPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl XcmTeleportPrecompile<Runtime> {
    #[precompile::public("teleportToRelayChain(bytes32,uint256)")]
    fn teleport_to_relay_chain(
        handle: &mut impl PrecompileHandle,
        destination_account: H256,
        amount: U256,
    ) -> EvmResult {
        // No benchmarks availabe yet for precompiles, so charge some arbitrary gas as a spam
        // prevention mechanism.
        handle.record_cost(1000)?;

        // We use IdentityAddressMapping, so no db access
        let account_id = <Runtime as pallet_evm::Config>::AddressMapping::into_account_id(
            handle.context().caller,
        );
        let origin: RuntimeOrigin = frame_system::RawOrigin::Signed(account_id).into();

        let destination = VersionedLocation::V5(RelayLocation::get());

        let beneficiary = VersionedLocation::V5(Location::new(
            0,
            [Junction::AccountId32 {
                network: None,
                id: destination_account.into(),
            }],
        ));

        let amount_u128: u128 = amount.try_into().map_err(|_| revert("Amount too large"))?;

        let assets = VersionedAssets::V5(Assets::from(vec![Asset {
            id: NativeAssetId::get(),
            fun: Fungibility::Fungible(amount_u128),
        }]));

        let fee_asset_item = 0;

        let call = pallet_xcm::Call::<Runtime>::teleport_assets {
            dest: Box::new(destination),
            beneficiary: Box::new(beneficiary),
            assets: Box::new(assets),
            fee_asset_item,
        };

        RuntimeHelper::<Runtime>::try_dispatch(handle, origin, call, 0)?;

        Ok(())
    }
}
