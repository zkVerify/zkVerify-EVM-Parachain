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

//! In this module, we provide the configurations about evm.

use crate::{
    constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO, WEIGHT_PER_GAS},
    opaque, weights, AccountId, Aura, Balances, BaseFee, DeploymentPermissions, EVMChainId,
    Permill, Precompiles, Runtime, RuntimeEvent, Timestamp, UncheckedExtrinsic,
};
use frame_support::{pallet_prelude::ConstU32, parameter_types, traits::FindAuthor};
use pallet_ethereum::PostLogContent;
use pallet_evm::{EVMCurrencyAdapter, EnsureAccountId20, IdentityAddressMapping};
use parity_scale_codec::{Decode, Encode};
use sp_core::{H160, U256};
use sp_runtime::ConsensusEngineId;
use sp_std::marker::PhantomData;
use sp_weights::Weight;

parameter_types! {
    pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StateRoot = pallet_ethereum::IntermediateStateRoot<Self::Version>;
    type PostLogContent = PostBlockAndTxnHashes;
    type ExtraDataLength = ConstU32<30>;
}

const MAX_STORAGE_GROWTH: u64 = 400 * 1024;

parameter_types! {
    pub BlockGasLimit: U256 = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
    pub GasLimitPovSizeRatio: u64 = BlockGasLimit::get().as_u64().saturating_div(cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64);
    pub GasLimitStorageGrowthRatio: u64 = BlockGasLimit::get().as_u64().saturating_div(MAX_STORAGE_GROWTH);
    pub PrecompilesValue: Precompiles<Runtime> = Precompiles::<_>::new();
    pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
}

impl pallet_deployment_permissions::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_deployment_permissions::ZKVEvmWeight<Self>;
}

type BaseRunner<T> = pallet_evm::runner::stack::Runner<T>;
type PermissionedRunner<T> = pallet_deployment_permissions::runner::PermissionedDeploy<
    T,
    BaseRunner<T>,
    DeploymentPermissions,
>;

impl pallet_evm::Config for Runtime {
    type AccountProvider = pallet_evm::FrameSystemAccountProvider<Self>;
    type FeeCalculator = BaseFee;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas;
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type CallOrigin = EnsureAccountId20;
    type WithdrawOrigin = EnsureAccountId20;
    type AddressMapping = IdentityAddressMapping;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type PrecompilesType = Precompiles<Self>;
    type PrecompilesValue = PrecompilesValue;
    type ChainId = EVMChainId;
    type BlockGasLimit = BlockGasLimit;
    type Runner = PermissionedRunner<Self>;
    type OnChargeTransaction = EVMCurrencyAdapter<Balances, ()>;
    type OnCreate = ();
    type FindAuthor = FindAuthorSession<pallet_session::FindAccountFromAuthorIndex<Self, Aura>>;
    type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
    type GasLimitStorageGrowthRatio = GasLimitStorageGrowthRatio;
    type Timestamp = Timestamp;
    type WeightInfo = weights::pallet_evm::ZKVEvmWeight<Self>;
}

impl pallet_evm_chain_id::Config for Runtime {}

parameter_types! {
    /// Starting value for base fee. Set at the same value as in Ethereum.
    pub DefaultBaseFeePerGas: U256 = U256::from(1_000_000_000);
    /// Default elasticity rate. Set at the same value as in Ethereum.
    pub DefaultElasticity: Permill = Permill::from_parts(125_000);
}

/// The thresholds based on which the base fee will change.
pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
    fn lower() -> Permill {
        Permill::zero()
    }

    fn ideal() -> Permill {
        Permill::from_parts(500_000)
    }

    fn upper() -> Permill {
        Permill::from_parts(1_000_000)
    }
}
impl pallet_base_fee::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Threshold = BaseFeeThreshold;
    type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
    type DefaultElasticity = DefaultElasticity;
}

pub struct FindAuthorSession<Inner>(PhantomData<Inner>);
impl<Inner> FindAuthor<H160> for FindAuthorSession<Inner>
where
    Inner: FindAuthor<AccountId>,
{
    fn find_author<'a, I>(digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        let tmp = Inner::find_author(digests);
        tmp.map(Into::into)
    }
}

#[derive(Clone)]
pub struct TransactionConverter;

impl fp_rpc::ConvertTransaction<UncheckedExtrinsic> for TransactionConverter {
    fn convert_transaction(&self, transaction: pallet_ethereum::Transaction) -> UncheckedExtrinsic {
        UncheckedExtrinsic::new_bare(
            pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
        )
    }
}

impl fp_rpc::ConvertTransaction<opaque::UncheckedExtrinsic> for TransactionConverter {
    fn convert_transaction(
        &self,
        transaction: pallet_ethereum::Transaction,
    ) -> opaque::UncheckedExtrinsic {
        let extrinsic = UncheckedExtrinsic::new_bare(
            pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
        );
        let encoded = extrinsic.encode();
        opaque::UncheckedExtrinsic::decode(&mut &encoded[..])
            .expect("Encoded extrinsic is always valid")
    }
}
