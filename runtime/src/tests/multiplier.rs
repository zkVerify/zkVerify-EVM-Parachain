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

// Integration transaction weight-fee tests
mod common;
use common::*;
use frame_support::pallet_prelude::*;
use zkv_para_evm_runtime::{Runtime, RuntimeBlockWeights};
use pallet_transaction_payment::Multiplier;
use polkadot_runtime_common::MinimumMultiplier;
use sp_runtime::{traits::Convert, Perquintill};

fn min_multiplier() -> Multiplier {
    MinimumMultiplier::get()
}

fn target() -> Weight {
    Perquintill::from_percent(25)
        * RuntimeBlockWeights::get()
            .get(DispatchClass::Normal)
            .max_total
            .unwrap()
}

fn runtime_multiplier_update(fm: Multiplier) -> Multiplier {
    <Runtime as pallet_transaction_payment::Config>::FeeMultiplierUpdate::convert(fm)
}

#[test]
fn multiplier_can_grow_from_zero() {
    // if the min is too small, then this will not change, and we are doomed forever.
    // the block ref time is 1/100th bigger than target.
    run_with_system_weight(
        target().set_ref_time(target().ref_time() * 101 / 100),
        || {
            let next = runtime_multiplier_update(min_multiplier());
            assert!(
                next > min_multiplier(),
                "{:?} !> {:?}",
                next,
                min_multiplier()
            );
        },
    );

    // the block proof size is 1/100th bigger than target.
    run_with_system_weight(
        target().set_proof_size((target().proof_size() / 100) * 101),
        || {
            let next = runtime_multiplier_update(min_multiplier());
            assert!(
                next > min_multiplier(),
                "{:?} !> {:?}",
                next,
                min_multiplier()
            );
        },
    )
}

#[test]
fn multiplier_cannot_go_below_limit() {
    // will not go any further below even if block is empty.
    run_with_system_weight(Weight::zero(), || {
        let next = runtime_multiplier_update(min_multiplier());
        assert_eq!(next, min_multiplier());
    })
}
