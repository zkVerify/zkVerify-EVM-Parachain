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

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod configs;
pub mod constants;
mod genesis_config_presets;

mod precompiles;
pub use precompiles::Precompiles;
#[cfg(test)]
mod tests;
pub mod types;
mod weights;
mod xcm_teleport;

/// In this module, we're re-export all dependencies needed by special weight modules.
pub(crate) mod weights_aliases {
    pub mod frame_system_extensions {
        pub use frame_system::ExtensionsWeightInfo as WeightInfo;
    }
}

#[macro_use]
extern crate alloc;

use alloc::{borrow::Cow, string::String};

use frame_support::{
    construct_runtime,
    genesis_builder_helper::{build_state, get_preset},
    traits::OnFinalize,
    weights::Weight,
};
use pallet_ethereum::{
    Call::transact, Transaction as EthereumTransaction, TransactionAction, TransactionData,
    TransactionStatus,
};
use pallet_evm::{Account as EVMAccount, FeeCalculator, Runner};
use sp_api::impl_runtime_apis;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H160, H256, U256};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
    impl_opaque_keys,
    traits::{
        Block as BlockT, DispatchInfoOf, Dispatchable, Get, PostDispatchInfoOf, UniqueSaturatedInto,
    },
    transaction_validity::{TransactionSource, TransactionValidity, TransactionValidityError},
    ApplyExtrinsicResult,
};
pub use sp_runtime::{Perbill, Permill};
use sp_std::prelude::{Vec, *};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub use crate::types::{
    AccountId, Balance, Block, BlockNumber, Executive, Nonce, Signature, UncheckedExtrinsic,
};
use crate::{constants::SLOT_DURATION, types::ConsensusHook};

impl fp_self_contained::SelfContainedCall for RuntimeCall {
    type SignedInfo = H160;

    fn is_self_contained(&self) -> bool {
        match self {
            RuntimeCall::Ethereum(call) => call.is_self_contained(),
            _ => false,
        }
    }

    fn check_self_contained(&self) -> Option<Result<Self::SignedInfo, TransactionValidityError>> {
        match self {
            RuntimeCall::Ethereum(call) => call.check_self_contained(),
            _ => None,
        }
    }

    fn validate_self_contained(
        &self,
        info: &Self::SignedInfo,
        dispatch_info: &DispatchInfoOf<RuntimeCall>,
        len: usize,
    ) -> Option<TransactionValidity> {
        match self {
            RuntimeCall::Ethereum(call) => call.validate_self_contained(info, dispatch_info, len),
            _ => None,
        }
    }

    fn pre_dispatch_self_contained(
        &self,
        info: &Self::SignedInfo,
        dispatch_info: &DispatchInfoOf<RuntimeCall>,
        len: usize,
    ) -> Option<Result<(), TransactionValidityError>> {
        match self {
            RuntimeCall::Ethereum(call) => {
                call.pre_dispatch_self_contained(info, dispatch_info, len)
            }
            _ => None,
        }
    }

    fn apply_self_contained(
        self,
        info: Self::SignedInfo,
    ) -> Option<sp_runtime::DispatchResultWithInfo<PostDispatchInfoOf<Self>>> {
        match self {
            call @ RuntimeCall::Ethereum(pallet_ethereum::Call::transact { .. }) => {
                Some(call.dispatch(RuntimeOrigin::from(
                    pallet_ethereum::RawOrigin::EthereumTransaction(info),
                )))
            }
            _ => None,
        }
    }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't
/// need to know the specifics of the runtime. They can then be made to be
/// agnostic over specific formats of data like extrinsics, allowing for them to
/// continue syncing the network through upgrades to even the core data
/// structures.
pub mod opaque {
    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    use sp_runtime::{
        generic,
        traits::{BlakeTwo256, Hash as HashT},
    };

    use super::*;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    /// Opaque block hash type.
    pub type Hash = <BlakeTwo256 as HashT>::Output;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
    }
}

// Version of the runtime.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: Cow::Borrowed("tvflow-runtime"),
    impl_name: Cow::Borrowed("vflow-node"),
    authoring_version: 1,
    spec_version: 2000,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmark;

// Create the runtime by composing the FRAME pallets that were previously
// configured.
construct_runtime!(
    pub enum Runtime
    {
        // System Support
        System: frame_system = 0,
        ParachainSystem: cumulus_pallet_parachain_system = 1,
        Timestamp: pallet_timestamp = 2,
        ParachainInfo: parachain_info = 3, // No weight
        Proxy: pallet_proxy = 4,
        Utility: pallet_utility = 5,
        Multisig: pallet_multisig = 6,

        // Monetary
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11, // No weight

        // Governance
        Sudo: pallet_sudo = 15,

        // Consensus
        // Collator Support. The order of these 5 are important and shall not change.
        Authorship: pallet_authorship = 20, // No weight
        CollatorSelection: pallet_collator_selection = 21,
        Session: pallet_session = 22,
        Aura: pallet_aura = 23, // No weight
        AuraExt: cumulus_pallet_aura_ext = 24, // No weight

        // XCM Helpers
        XcmpQueue: cumulus_pallet_xcmp_queue = 30,
        ZKVXcm: pallet_xcm = 31,
        CumulusXcm: cumulus_pallet_xcm = 32, // No weight
        MessageQueue: pallet_message_queue = 33,

        // EVM
        Ethereum: pallet_ethereum = 40, // No weight
        EVM: pallet_evm = 41,
        EVMChainId: pallet_evm_chain_id = 43, // No weight
        EthereumXcm: pallet_ethereum_xcm = 44,

        // zkVerify Custom Pallets
        DeploymentPermissions: pallet_deployment_permissions = 100,
    }
);

// this has to be here due to linking in the marco,
// it cannot be extracted into separate file
impl_runtime_apis! {
    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(SLOT_DURATION)
        }

        fn authorities() -> Vec<AuraId> {
            pallet_aura::Authorities::<Runtime>::get().into_inner()
        }
    }

    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
        for Runtime
    {
        fn query_call_info(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_call_info(call, len)
        }
        fn query_call_fee_details(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_call_fee_details(call, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl fp_rpc::EthereumRuntimeRPCApi<Block> for Runtime {
        /// Returns runtime defined pallet_evm::ChainId.
        fn chain_id() -> u64 {
            <Runtime as pallet_evm::Config>::ChainId::get()
        }

        /// Returns pallet_evm::Accounts by address.
        fn account_basic(address: H160) -> EVMAccount {
            let (account, _) = pallet_evm::Pallet::<Runtime>::account_basic(&address);
            account
        }

        /// Returns FixedGasPrice::min_gas_price
        fn gas_price() -> U256 {
            let (gas_price, _) = <Runtime as pallet_evm::Config>::FeeCalculator::min_gas_price();
            gas_price
        }

        /// For a given account address, returns pallet_evm::AccountCodes.
        fn account_code_at(address: H160) -> Vec<u8> {
            pallet_evm::AccountCodes::<Runtime>::get(address)
        }

        /// Returns the converted FindAuthor::find_author authority id.
        fn author() -> H160 {
            <pallet_evm::Pallet<Runtime>>::find_author()
        }

        /// For a given account address and index, returns pallet_evm::AccountStorages.
        fn storage_at(address: H160, index: U256) -> H256 {
            let tmp = index.to_big_endian();
            pallet_evm::AccountStorages::<Runtime>::get(address, H256::from_slice(&tmp[..]))
        }

        /// Returns a frame_ethereum::call response.
        fn call(
            from: H160,
            to: H160,
            data: Vec<u8>,
            value: U256,
            gas_limit: U256,
            max_fee_per_gas: Option<U256>,
            max_priority_fee_per_gas: Option<U256>,
            nonce: Option<U256>,
            estimate: bool,
            access_list: Option<Vec<(H160, Vec<H256>)>>,
        ) -> Result<pallet_evm::CallInfo, sp_runtime::DispatchError> {
            let config = if estimate {
                let mut config = <Runtime as pallet_evm::Config>::config().clone();
                config.estimate = true;
                Some(config)
            } else {
                None
            };

            let gas_limit = gas_limit.min(u64::MAX.into());
            let transaction_data = TransactionData::new(
                TransactionAction::Call(to),
                data.clone(),
                nonce.unwrap_or_default(),
                gas_limit,
                None,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                value,
                Some(<Runtime as pallet_evm::Config>::ChainId::get()),
                access_list.clone().unwrap_or_default(),
            );
            let (weight_limit, proof_size_base_cost) = pallet_ethereum::Pallet::<Runtime>::transaction_weight(&transaction_data);

            <Runtime as pallet_evm::Config>::Runner::call(
                from,
                to,
                data,
                value,
                gas_limit.unique_saturated_into(),
                max_fee_per_gas,
                max_priority_fee_per_gas,
                nonce,
                access_list.unwrap_or_default(),
                false,
                true,
                weight_limit,
                proof_size_base_cost,
                config.as_ref().unwrap_or(<Runtime as pallet_evm::Config>::config()),
            ).map_err(|err| err.error.into())
        }

        /// Returns a frame_ethereum::create response.
        fn create(
            from: H160,
            data: Vec<u8>,
            value: U256,
            gas_limit: U256,
            max_fee_per_gas: Option<U256>,
            max_priority_fee_per_gas: Option<U256>,
            nonce: Option<U256>,
            estimate: bool,
            access_list: Option<Vec<(H160, Vec<H256>)>>,
        ) -> Result<pallet_evm::CreateInfo, sp_runtime::DispatchError> {
            let config = if estimate {
                let mut config = <Runtime as pallet_evm::Config>::config().clone();
                config.estimate = true;
                Some(config)
            } else {
                None
            };

            let transaction_data = TransactionData::new(
                TransactionAction::Create,
                data.clone(),
                nonce.unwrap_or_default(),
                gas_limit,
                None,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                value,
                Some(<Runtime as pallet_evm::Config>::ChainId::get()),
                access_list.clone().unwrap_or_default(),
            );
            let (weight_limit, proof_size_base_cost) = pallet_ethereum::Pallet::<Runtime>::transaction_weight(&transaction_data);

            <Runtime as pallet_evm::Config>::Runner::create(
                from,
                data,
                value,
                gas_limit.unique_saturated_into(),
                max_fee_per_gas,
                max_priority_fee_per_gas,
                nonce,
                access_list.unwrap_or_default(),
                false,
                true,
                weight_limit,
                proof_size_base_cost,
                config.as_ref().unwrap_or(<Runtime as pallet_evm::Config>::config()),
            ).map_err(|err| err.error.into())
        }

        /// Return the current transaction status.
        fn current_transaction_statuses() -> Option<Vec<TransactionStatus>> {
            pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
        }

        /// Return the current block.
        fn current_block() -> Option<pallet_ethereum::Block> {
            pallet_ethereum::CurrentBlock::<Runtime>::get()
        }

        /// Return the current receipts.
        fn current_receipts() -> Option<Vec<pallet_ethereum::Receipt>> {
            pallet_ethereum::CurrentReceipts::<Runtime>::get()
        }

        /// Return all the current data for a block in a single runtime call.
        fn current_all() -> (
            Option<pallet_ethereum::Block>,
            Option<Vec<pallet_ethereum::Receipt>>,
            Option<Vec<TransactionStatus>>
        ) {
            (
                pallet_ethereum::CurrentBlock::<Runtime>::get(),
                pallet_ethereum::CurrentReceipts::<Runtime>::get(),
                pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
            )
        }

        /// Receives a `Vec<OpaqueExtrinsic>` and filters out all the non-ethereum transactions.
        fn extrinsic_filter(
            xts: Vec<<Block as BlockT>::Extrinsic>,
        ) -> Vec<EthereumTransaction> {
            xts.into_iter().filter_map(|xt| match xt.0.function {
                RuntimeCall::Ethereum(transact { transaction }) => Some(transaction),
                _ => None
            }).collect::<Vec<EthereumTransaction>>()
        }

        /// Return the elasticity multiplier.
        fn elasticity() -> Option<Permill> {
            None
        }

        /// Used to determine if gas limit multiplier for non-transactional calls (eth_call/estimateGas)
        /// is supported.
        fn gas_limit_multiplier_support() {}

        /// Return the pending block.
        fn pending_block(
            xts: Vec<<Block as BlockT>::Extrinsic>,
        ) -> (Option<pallet_ethereum::Block>, Option<Vec<TransactionStatus>>) {
            for ext in xts.into_iter() {
                let _ = Executive::apply_extrinsic(ext);
            }

            Ethereum::on_finalize(System::block_number() + 1);

            (
                pallet_ethereum::CurrentBlock::<Runtime>::get(),
                pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
            )
        }

        fn initialize_pending_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header);
        }
    }

    impl fp_rpc::ConvertTransactionRuntimeApi<Block> for Runtime {
        /// Converts an ethereum transaction into a transaction suitable for the runtime.
        fn convert_transaction(transaction: EthereumTransaction) -> <Block as BlockT>::Extrinsic {
            UncheckedExtrinsic::new_bare(
                pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
            )
        }
    }



    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
            ParachainSystem::collect_collation_info(header)
        }
    }

    impl cumulus_primitives_aura::AuraUnincludedSegmentApi<Block> for Runtime {
        fn can_build_upon(
            included_hash: <Block as BlockT>::Hash,
            slot: cumulus_primitives_aura::Slot
        ) -> bool {
            ConsensusHook::can_build_upon(included_hash, slot)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            use configs::system::RuntimeBlockWeights;

            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, RuntimeBlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect,
        ) -> Weight {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here.
            Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use frame_system_benchmarking::extensions::Pallet as SystemExtensionsBench;
            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

            use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;

            pub mod xcm {
                pub use pallet_xcm_benchmarks::fungible::Pallet as XcmPalletBenchFungible;
                pub use pallet_xcm_benchmarks::generic::Pallet as XcmPalletBenchGeneric;
            }

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();

            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, String> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch};
            use frame_system_benchmarking::Pallet as SystemBench;
            use frame_system_benchmarking::extensions::Pallet as SystemExtensionsBench;
            use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;

            pub mod xcm {
                use super::*;
                use crate::{configs::monetary::*, configs::xcm::*, constants::currency::CENTS};
                use frame_support::parameter_types;
                use xcm::v5::{Asset, AssetId, Assets, Location, InteriorLocation, Junction, Junctions::Here, NetworkId, Response, Fungibility::Fungible, Parent};
                use frame_benchmarking::BenchmarkError;

                pub use pallet_xcm_benchmarks::fungible::Pallet as XcmPalletBenchFungible;
                pub use pallet_xcm_benchmarks::generic::Pallet as XcmPalletBenchGeneric;

                parameter_types! {
                    pub ExistentialDepositAsset: Option<Asset> = Some((
                        RelayLocation::get(),
                        configs::monetary::ExistentialDeposit::get()
                    ).into());
                    /// The base fee for the message delivery fees. Kusama is based for the reference.
                    pub const ToParentBaseDeliveryFee: u128 = CENTS.saturating_mul(3);
                }

                impl pallet_xcm::benchmarking::Config for Runtime {
                    type DeliveryHelper = cumulus_primitives_utility::ToParentDeliveryHelper<
                        XcmConfig,
                        ExistentialDepositAsset,
                        PriceForParentDelivery,
                    >;

                    fn reachable_dest() -> Option<Location> {
                        Some(Parent.into())
                    }

                    fn teleportable_asset_and_dest() -> Option<(Asset, Location)> {
                        // Relay/native token can be teleported between EVM and Relay.
                        Some((
                            ExistentialDepositAsset::get()?,
                            Parent.into(),
                        ))
                    }

                    fn reserve_transferable_asset_and_dest() -> Option<(Asset, Location)> {
                        // Reserve transfers are disabled on EVM.
                        None
                    }

                    fn set_up_complex_asset_transfer(
                    ) -> Option<(Assets, u32, Location, Box<dyn FnOnce()>)> {
                        None
                    }

                    fn get_asset() -> Asset {
                        Asset {
                            id: NativeAssetId::get(),
                            fun: Fungible(ExistentialDeposit::get()),
                        }
                    }
                }

                impl pallet_xcm_benchmarks::Config for Runtime {
                    type XcmConfig = XcmConfig;
                    type AccountIdConverter = LocationAccountId32ToAccountId;
                    type DeliveryHelper = cumulus_primitives_utility::ToParentDeliveryHelper<
                        XcmConfig,
                        ExistentialDepositAsset,
                        PriceForParentDelivery,
                    >;

                    fn valid_destination() -> Result<Location, BenchmarkError> {
                        Ok(Location::parent())
                    }
                    fn worst_case_holding(_depositable_count: u32) -> Assets {
                        vec![Asset {
                            id: FeeAssetId::get(),
                            fun: Fungible(crate::constants::currency::tVFY),
                        }].into()
                    }
                }

                parameter_types! {
                    pub TrustedTeleporter: Option<(Location, Asset)> = Some((
                        RelayLocation::get(),
                        Asset {
                            id: NativeAssetId::get(),
                            fun: Fungible(ExistentialDeposit::get()),
                        },
                    ));
                    pub const TrustedReserve: Option<(Location, Asset)> = None;
                    pub const CheckedAccount: Option<(AccountId, xcm_builder::MintLocation)> = None;
                }

                impl pallet_xcm_benchmarks::fungible::Config for Runtime {
                    type TransactAsset = Balances;
                    type CheckedAccount = CheckedAccount;
                    type TrustedTeleporter = TrustedTeleporter;
                    type TrustedReserve = TrustedReserve;

                    fn get_asset() -> Asset {
                        Asset {
                            id: NativeAssetId::get(),
                            fun: Fungible(ExistentialDeposit::get()),
                        }
                    }
                }

                impl pallet_xcm_benchmarks::generic::Config for Runtime {
                    type TransactAsset = Balances;
                    type RuntimeCall = RuntimeCall;

                    fn worst_case_response() -> (u64, Response) {
                        (0u64, Response::Version(Default::default()))
                    }

                    fn worst_case_asset_exchange() -> Result<(Assets, Assets), BenchmarkError> {
                        // ZKV doesn't support asset exchanges
                        Err(BenchmarkError::Skip)
                    }

                    fn universal_alias() -> Result<(Location, Junction), BenchmarkError> {
                        // The XCM executor of ZKV doesn't have a configured `UniversalAliases`
                        Err(BenchmarkError::Skip)
                    }

                    fn transact_origin_and_runtime_call() -> Result<(Location, RuntimeCall), BenchmarkError> {
                        Ok((RelayLocation::get(), frame_system::Call::remark_with_event { remark: vec![] }.into()))
                    }

                    fn subscribe_origin() -> Result<Location, BenchmarkError> {
                        Ok(RelayLocation::get())
                    }

                    fn claimable_asset() -> Result<(Location, Location, Assets), BenchmarkError> {
                        // an asset that can be trapped and claimed
                        use crate::constants::currency::tVFY;
                        let origin = RelayLocation::get();
                        let assets: Assets = (NativeAssetId::get(), tVFY).into();
                        let ticket = Location { parents: 0, interior: Here };
                        Ok((origin, ticket, assets))
                    }

                    fn fee_asset() -> Result<Asset, BenchmarkError> {
                        Ok(Asset {
                            id: FeeAssetId::get(),
                            fun: Fungible(CENTS),
                        })
                    }

                    fn unlockable_asset() -> Result<(Location, Location, Asset), BenchmarkError> {
                        // ZKV doesn't support asset locking
                        Err(BenchmarkError::Skip)
                    }

                    fn export_message_origin_and_destination(
                    ) -> Result<(Location, NetworkId, InteriorLocation), BenchmarkError> {
                        // ZKV doesn't support exporting messages
                        Err(BenchmarkError::Skip)
                    }

                    fn alias_origin() -> Result<(Location, Location), BenchmarkError> {
                        // The XCM executor of ZKV doesn't have a configured `Aliasers`
                        Err(BenchmarkError::Skip)
                    }
                }
            }

            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

            use frame_support::traits::WhitelistedStorageKeys;
            let whitelist = AllPalletsWithSystem::whitelisted_storage_keys();

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, &genesis_config_presets::get_preset)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
           genesis_config_presets::preset_names()
        }
    }
}

#[cfg(feature = "runtime-benchmarks")]
mod runtime_benchmarking_extra_config {
    use crate::{ParachainSystem, Runtime, System};
    use frame_benchmarking::BenchmarkError;

    impl frame_system_benchmarking::Config for Runtime {
        fn setup_set_code_requirements(code: &sp_std::vec::Vec<u8>) -> Result<(), BenchmarkError> {
            ParachainSystem::initialize_for_set_code_benchmark(code.len() as u32);
            Ok(())
        }

        fn verify_set_code() {
            System::assert_last_event(
                cumulus_pallet_parachain_system::Event::<Runtime>::ValidationFunctionStored.into(),
            );
        }
    }

    impl cumulus_pallet_session_benchmarking::Config for Runtime {}
}
