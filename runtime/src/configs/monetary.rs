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

//! In this module we provide the configurations about monetary: the set
//! of pallets used for currency management.

use crate::{
    constants::currency::{EXISTENTIAL_DEPOSIT, MICROCENTS},
    weights, AccountId, Balance, Balances, CollatorSelection, Runtime, RuntimeEvent,
    RuntimeFreezeReason, RuntimeHoldReason, System, WeightToFee,
};
use frame_support::{parameter_types, traits::tokens::imbalance::ResolveTo};
use polkadot_runtime_common::SlowAdjustingFeeUpdate;
use sp_weights::ConstantMultiplier;

parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub const MaxFreezes: u32 = 0;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type WeightInfo = weights::pallet_balances::ZKVEvmWeight<Runtime>;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = ();
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type MaxFreezes = MaxFreezes;
    type DoneSlashHandler = ();
}

parameter_types! {
    /// Relay Chain `TransactionByteFee` / 10
    pub const TransactionByteFee: Balance = 10 * MICROCENTS;
    pub const OperationalFeeMultiplier: u8 = 5;
    pub StakingPot: AccountId = CollatorSelection::account_id();
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // All the fees go to the collators, passing through the Pot of the CollatorSelection pallet
    type OnChargeTransaction =
        pallet_transaction_payment::FungibleAdapter<Balances, ResolveTo<StakingPot, Balances>>;

    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
    type WeightInfo = weights::pallet_transaction_payment::ZKVEvmWeight<Runtime>;
}
