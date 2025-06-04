use frame_support::{construct_runtime, derive_impl, traits::Currency};
use sp_core::{H160, U256};
use sp_runtime::BuildStorage;

pub type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountIdOf<T> =
    <<T as pallet_evm::Config>::AccountProvider as pallet_evm::AccountProvider>::AccountId;
pub type BalanceOf<T> = <<T as pallet_evm::Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system = 0,
        Timestamp: pallet_timestamp = 1
        Balances: pallet_balances = 2,
        Evm: pallet_evm = 3,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = sp_runtime::AccountId32;
    type AccountData = pallet_balances::AccountData<u64>;
    type Lookup = sp_runtime::traits::IdentityLookup<sp_runtime::AccountId32>;
}

#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig)]
impl pallet_timestamp::Config for Test {}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type AccountStore = System;
}

#[derive_impl(pallet_evm::config_preludes::TestDefaultConfig)]
impl pallet_evm::Config for Test {
    type AccountProvider = pallet_evm::FrameSystemAccountProvider<Self>;
    type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
    type Timestamp = Timestamp;
    type Currency = Balances;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
}

// Test externalities initialization
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
