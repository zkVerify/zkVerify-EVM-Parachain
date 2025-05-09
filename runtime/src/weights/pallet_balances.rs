// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-04-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// target/release/zkv-para-evm-node
// benchmark
// pallet
// --genesis-builder=spec
// --pallet
// pallet-balances
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/mdamico/devel/zkVerify-EVM-Parachain/HEADER-APACHE2
// --output
// /home/mdamico/devel/zkVerify-EVM-Parachain/runtime/src/weights/pallet_balances.rs
// --template
// /home/mdamico/devel/zkVerify-EVM-Parachain/scripts/templates/deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_balances` using the zkVerify node and recommended hardware.
pub struct ZKVEvmWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_balances::WeightInfo for ZKVEvmWeight<T> {
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn transfer_allow_death() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `72`
        //  Estimated: `3581`
        // Minimum execution time: 91_675_000 picoseconds.
        Weight::from_parts(93_876_000, 3581)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn transfer_keep_alive() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `72`
        //  Estimated: `3581`
        // Minimum execution time: 71_203_000 picoseconds.
        Weight::from_parts(72_401_000, 3581)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn force_set_balance_creating() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `128`
        //  Estimated: `3581`
        // Minimum execution time: 26_756_000 picoseconds.
        Weight::from_parts(28_092_000, 3581)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn force_set_balance_killing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `128`
        //  Estimated: `3581`
        // Minimum execution time: 38_851_000 picoseconds.
        Weight::from_parts(39_463_000, 3581)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn force_transfer() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `200`
        //  Estimated: `6172`
        // Minimum execution time: 86_891_000 picoseconds.
        Weight::from_parts(88_963_000, 6172)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn transfer_all() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `72`
        //  Estimated: `3581`
        // Minimum execution time: 81_516_000 picoseconds.
        Weight::from_parts(82_911_000, 3581)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    fn force_unreserve() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `128`
        //  Estimated: `3581`
        // Minimum execution time: 29_676_000 picoseconds.
        Weight::from_parts(30_299_000, 3581)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::Account` (r:999 w:999)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// The range of component `u` is `[1, 1000]`.
    fn upgrade_accounts(u: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + u * (124 ±0)`
        //  Estimated: `990 + u * (2591 ±0)`
        // Minimum execution time: 27_907_000 picoseconds.
        Weight::from_parts(28_242_000, 990)
            // Standard Error: 9_490
            .saturating_add(Weight::from_parts(14_809_367, 0).saturating_mul(u.into()))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u.into())))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
            .saturating_add(Weight::from_parts(0, 2591).saturating_mul(u.into()))
    }
    fn force_adjust_total_issuance() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 7_065_000 picoseconds.
        Weight::from_parts(7_209_000, 0)
    }
    fn burn_allow_death() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 34_409_000 picoseconds.
        Weight::from_parts(35_494_000, 0)
    }
    fn burn_keep_alive() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 23_978_000 picoseconds.
        Weight::from_parts(24_702_000, 0)
    }
}
