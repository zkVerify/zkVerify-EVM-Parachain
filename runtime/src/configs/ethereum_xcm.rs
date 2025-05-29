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

//! In this module, we provide the configurations for the ethereum-xcm pallet.

use crate::{
    configs::system::{ProxyType, ReservedXcmpWeight, RuntimeBlockWeights},
    constants::currency::CENTS,
    types::AccountId,
    weights, AllPalletsWithSystem, Balances, BlockNumber, MessageQueue, ParachainInfo,
    ParachainSystem, Perbill, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, WeightToFee,
    XcmpQueue, ZKVXcm,
};
use cumulus_primitives_core::{AggregateMessageOrigin, ParaId};
use frame_support::{
    ensure,
    pallet_prelude::Get,
    parameter_types,
    traits::tokens::imbalance::ResolveTo,
    traits::OriginTrait,
    traits::TransformOrigin,
    traits::{ConstU32, Contains, Equals, Everything, Nothing, PalletInfoAccess},
    weights::Weight,
};
use frame_system::EnsureRoot;
use pallet_xcm::XcmPassthrough;
use parachains_common::{
    message_queue::{NarrowOriginToSibling, ParaIdToSibling},
    xcm_config::ConcreteAssetFromSystem,
};
use sp_runtime::traits::{TryConvert, Zero};
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountKey20Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
    AllowTopLevelPaidExecutionFrom, DenyReserveTransferToRelayChain, DenyThenTry,
    DescribeAllTerminal, DescribeFamily, EnsureXcmOrigin, FrameTransactionalProcessor,
    FungibleAdapter, HashedDescription, IsConcrete, NativeAsset, ParentIsPreset,
    RelayChainAsNative, SendXcmFeeToAccount, SiblingParachainAsNative, SignedAccountKey20AsNative,
    SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId, UsingComponents,
    WeightInfoBounds, WithComputedOrigin, WithUniqueTopic, XcmFeeManagerFromComponents,
};
use xcm_executor::{traits::ConvertLocation, XcmExecutor};

pub struct EthereumXcmEnsureProxy;
impl xcm_primitives::EnsureProxy<AccountId> for EthereumXcmEnsureProxy {
    fn ensure_ok(delegator: AccountId, delegatee: AccountId) -> Result<(), &'static str> {
        // The EVM implicitly contains an Any proxy, so we only allow for "Any" proxies
        let def: pallet_proxy::ProxyDefinition<AccountId, ProxyType, BlockNumber> =
            pallet_proxy::Pallet::<Runtime>::find_proxy(
                &delegator,
                &delegatee,
                Some(ProxyType::Any),
            )
            .map_err(|_| "proxy error: expected `ProxyType::Any`")?;
        // We only allow to use it for delay zero proxies, as the call will immediatly be executed
        ensure!(def.delay.is_zero(), "proxy delay is Non-zero`");
        Ok(())
    }
}

impl pallet_ethereum_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type InvalidEvmTransactionError = pallet_ethereum::InvalidTransactionWrapper;
    type ValidatedTransaction = pallet_ethereum::ValidatedTransaction<Self>;
    type XcmEthereumOrigin = pallet_ethereum_xcm::EnsureXcmEthereumTransaction;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type EnsureProxy = EthereumXcmEnsureProxy;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ForceOrigin = EnsureRoot<AccountId>;
}
