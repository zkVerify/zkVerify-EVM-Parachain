use sp_core::{H160, U256};
use sp_runtime::{traits::{BlakeTwo256, IdentityLookup}, BuildStorage};
use frame_support::{parameter_types, traits::{Everything}, weights::Weight};
use frame_system as frame_system;
use pallet_evm::{EnsureAddressNever, IdentityAddressMapping, EVMCurrencyAdapter};
use crate::Precompiles;
use pallet_evm::config_preludes::FixedGasPrice;

pub type AccountId = H160;
pub type BlockNumber = u64;
pub type Balance = u128;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Nonce = u64;
    type Block = Block;
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type Version = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
    pub const MaxLocks: u32 = 50;
}
impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Pallet<Test>;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
}
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub BlockGasLimit: U256 = U256::from(u32::MAX);
    pub static ChainId: u64 = 1;
    pub WeightPerGas: Weight = Weight::from_parts(1, 0);
}

use frame_support::traits::Get;
pub struct TestPrecompiles;
impl Get<Precompiles<Test>> for TestPrecompiles {
    fn get() -> Precompiles<Test> {
        Precompiles::<Test>::new()
    }
}
impl pallet_evm::Config for Test {
    type AccountProvider = pallet_evm::FrameSystemAccountProvider<Self>;
    type FeeCalculator = FixedGasPrice;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas;
    type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressNever<AccountId>;
    type WithdrawOrigin = EnsureAddressNever<AccountId>;
    type AddressMapping = IdentityAddressMapping;
    type Currency = pallet_balances::Pallet<Self>;
    type RuntimeEvent = RuntimeEvent;
    type PrecompilesType = Precompiles<Self>;
    type PrecompilesValue = TestPrecompiles;
    type ChainId = ChainId;
    type BlockGasLimit = BlockGasLimit;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type OnChargeTransaction = EVMCurrencyAdapter<pallet_balances::Pallet<Self>, ()>;
    type OnCreate = ();
    type FindAuthor = ();
    type GasLimitPovSizeRatio = ();
    type GasLimitStorageGrowthRatio = ();
    type Timestamp = pallet_timestamp::Pallet<Self>;
    type WeightInfo = ();
}

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        EVM: pallet_evm,
    }
);

pub type Block = frame_system::mocking::MockBlock<Test>;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    let mut ext = sp_io::TestExternalities::from(t);
    ext.execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        pallet_timestamp::Pallet::<Test>::set_timestamp(0);
    });
    ext
}
