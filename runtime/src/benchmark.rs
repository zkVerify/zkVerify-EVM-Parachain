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

frame_benchmarking::define_benchmarks!(
    [frame_system, SystemBench::<Runtime>]
    [cumulus_pallet_parachain_system, ParachainSystem]
    [pallet_timestamp, Timestamp]
    [pallet_proxy, Proxy]
    [pallet_utility, Utility]
    [pallet_multisig, Multisig]

    [pallet_balances, Balances]
    [pallet_assets, Assets]

    [pallet_sudo, Sudo]

    [pallet_colleator_selection, CollatorSelection]
    [pallet_session, SessionBench::<Runtime>]

    [cumulus_pallet_xcmp_queue, XcmpQueue]
    [pallet_xcm, PalletXcmExtrinsicsBenchmark::<Runtime>]
    [pallet_message_queue, MessageQueue]

    [pallet_evm, EVM]
);

use cumulus_primitives_core::{ChannelStatus, GetChannelInfo};
use frame_support::traits::{
    tokens::{Pay, PaymentStatus},
    Get,
};
use sp_std::marker::PhantomData;

use crate::ParachainSystem;

/// Trait for setting up any prerequisites for successful execution of benchmarks.
pub trait EnsureSuccessful {
    fn ensure_successful();
}

/// Implementation of the [`EnsureSuccessful`] trait which opens an HRMP channel between
/// the Collectives and a parachain with a given ID.
pub struct OpenHrmpChannel<I>(PhantomData<I>);
impl<I: Get<u32>> EnsureSuccessful for OpenHrmpChannel<I> {
    fn ensure_successful() {
        if let ChannelStatus::Closed = ParachainSystem::get_channel_status(I::get().into()) {
            ParachainSystem::open_outbound_hrmp_channel_for_benchmarks_or_tests(I::get().into())
        }
    }
}

/// Type that wraps a type implementing the [`Pay`] trait to decorate its
/// [`Pay::ensure_successful`] function with a provided implementation of the
/// [`EnsureSuccessful`] trait.
pub struct PayWithEnsure<O, E>(PhantomData<(O, E)>);
impl<O, E> Pay for PayWithEnsure<O, E>
where
    O: Pay,
    E: EnsureSuccessful,
{
    type AssetKind = O::AssetKind;
    type Balance = O::Balance;
    type Beneficiary = O::Beneficiary;
    type Error = O::Error;
    type Id = O::Id;

    fn pay(
        who: &Self::Beneficiary,
        asset_kind: Self::AssetKind,
        amount: Self::Balance,
    ) -> Result<Self::Id, Self::Error> {
        O::pay(who, asset_kind, amount)
    }

    fn check_payment(id: Self::Id) -> PaymentStatus {
        O::check_payment(id)
    }

    fn ensure_successful(
        who: &Self::Beneficiary,
        asset_kind: Self::AssetKind,
        amount: Self::Balance,
    ) {
        E::ensure_successful();
        O::ensure_successful(who, asset_kind, amount)
    }

    fn ensure_concluded(id: Self::Id) {
        O::ensure_concluded(id)
    }
}
