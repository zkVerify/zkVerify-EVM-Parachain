use alloc::vec::Vec;
use core::marker::PhantomData;
use frame_support::{sp_runtime::DispatchError, weights::Weight};
use pallet_evm::{
    runner::Runner as RunnerT, BalanceOf, Config, EvmConfig, FeeCalculator, RunnerError,
};
use sp_core::{H160, H256, U256};

use crate::EnsureCreateOrigin;

#[derive(Default)]
pub struct PermissionedDeploy<T, R, C> {
    _marker: PhantomData<(T, R, C)>,
}

#[derive(Debug)]
pub enum PermissionedDeployError<T, R, C>
where
    T: Config,
    R: RunnerT<T>,
    C: EnsureCreateOrigin<T>,
{
    Runner(R::Error),
    Permission(C::Error),
}

impl<T, R, C> From<PermissionedDeployError<T, R, C>> for DispatchError
where
    T: Config,
    R: RunnerT<T>,
    C: EnsureCreateOrigin<T>,
{
    fn from(value: PermissionedDeployError<T, R, C>) -> Self {
        match value {
            PermissionedDeployError::Runner(error) => error.into(),
            PermissionedDeployError::Permission(error) => error.into(),
        }
    }
}

impl<T, R, C> RunnerT<T> for PermissionedDeploy<T, R, C>
where
    T: Config,
    R: RunnerT<T>,
    C: EnsureCreateOrigin<T>,
    BalanceOf<T>: TryFrom<U256> + Into<U256>,
{
    type Error = PermissionedDeployError<T, R, C>;

    fn validate(
        source: H160,
        target: Option<H160>,
        input: Vec<u8>,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
        nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
        is_transactional: bool,
        weight_limit: Option<Weight>,
        proof_size_base_cost: Option<u64>,
        evm_config: &EvmConfig,
    ) -> Result<(), pallet_evm::RunnerError<Self::Error>> {
        R::validate(
            source,
            target,
            input,
            value,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            nonce,
            access_list,
            is_transactional,
            weight_limit,
            proof_size_base_cost,
            evm_config,
        )
        .map_err(|err| RunnerError {
            error: PermissionedDeployError::Runner(err.error),
            weight: err.weight,
        })
    }

    fn call(
        source: H160,
        target: H160,
        input: Vec<u8>,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
        nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
        is_transactional: bool,
        validate: bool,
        weight_limit: Option<Weight>,
        proof_size_base_cost: Option<u64>,
        config: &EvmConfig,
    ) -> Result<pallet_evm::CallInfo, pallet_evm::RunnerError<Self::Error>> {
        R::call(
            source,
            target,
            input,
            value,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            nonce,
            access_list,
            is_transactional,
            validate,
            weight_limit,
            proof_size_base_cost,
            config,
        )
        .map_err(|err| RunnerError {
            error: PermissionedDeployError::Runner(err.error),
            weight: err.weight,
        })
    }

    fn create(
        source: H160,
        init: Vec<u8>,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
        nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
        is_transactional: bool,
        validate: bool,
        weight_limit: Option<Weight>,
        proof_size_base_cost: Option<u64>,
        config: &EvmConfig,
    ) -> Result<pallet_evm::CreateInfo, pallet_evm::RunnerError<Self::Error>> {
        let (_, weight) = T::FeeCalculator::min_gas_price();
        C::check_create_origin(&source).map_err(|err| RunnerError {
            error: PermissionedDeployError::Permission(err),
            weight,
        })?;

        R::create(
            source,
            init,
            value,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            nonce,
            access_list,
            is_transactional,
            validate,
            weight_limit,
            proof_size_base_cost,
            config,
        )
        .map_err(|err| RunnerError {
            error: PermissionedDeployError::Runner(err.error),
            weight: err.weight,
        })
    }

    fn create2(
        source: H160,
        init: Vec<u8>,
        salt: H256,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
        nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
        is_transactional: bool,
        validate: bool,
        weight_limit: Option<Weight>,
        proof_size_base_cost: Option<u64>,
        config: &EvmConfig,
    ) -> Result<pallet_evm::CreateInfo, pallet_evm::RunnerError<Self::Error>> {
        let (_, weight) = T::FeeCalculator::min_gas_price();
        C::check_create_origin(&source).map_err(|err| RunnerError {
            error: PermissionedDeployError::Permission(err),
            weight,
        })?;

        R::create2(
            source,
            init,
            salt,
            value,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            nonce,
            access_list,
            is_transactional,
            validate,
            weight_limit,
            proof_size_base_cost,
            config,
        )
        .map_err(|err| RunnerError {
            error: PermissionedDeployError::Runner(err.error),
            weight: err.weight,
        })
    }
}
