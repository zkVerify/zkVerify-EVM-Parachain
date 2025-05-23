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

//! Autogenerated weights for `frame_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-04-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// /home/mdamico/devel/zkVerify-EVM-Parachain/target/release/zkv-evm-para-node
// benchmark
// pallet
// --genesis-builder=spec
// --pallet
// frame-system
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
// /home/mdamico/devel/zkVerify-EVM-Parachain/runtime/src/weights/frame_system.rs
// --template
// /home/mdamico/devel/zkVerify-EVM-Parachain/scripts/templates/deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `frame_system` using the zkVerify node and recommended hardware.
pub struct ZKVEvmWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> frame_system::WeightInfo for ZKVEvmWeight<T> {
    /// The range of component `b` is `[0, 3932160]`.
    fn remark(b: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_813_000 picoseconds.
        Weight::from_parts(3_969_000, 0)
            // Standard Error: 95
            .saturating_add(Weight::from_parts(8_476, 0).saturating_mul(b.into()))
    }
    /// The range of component `b` is `[0, 3932160]`.
    fn remark_with_event(b: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 6_169_000 picoseconds.
        Weight::from_parts(6_306_000, 0)
            // Standard Error: 96
            .saturating_add(Weight::from_parts(9_374, 0).saturating_mul(b.into()))
    }
    /// Storage: `System::Digest` (r:1 w:1)
    /// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: UNKNOWN KEY `0x3a686561707061676573` (r:0 w:1)
    /// Proof: UNKNOWN KEY `0x3a686561707061676573` (r:0 w:1)
    fn set_heap_pages() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `1485`
        // Minimum execution time: 4_074_000 picoseconds.
        Weight::from_parts(4_220_000, 1485)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
    /// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::UpgradeRestrictionSignal` (r:1 w:0)
    /// Proof: `ParachainSystem::UpgradeRestrictionSignal` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::PendingValidationCode` (r:1 w:1)
    /// Proof: `ParachainSystem::PendingValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
    /// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::NewValidationCode` (r:0 w:1)
    /// Proof: `ParachainSystem::NewValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::DidSetValidationCode` (r:0 w:1)
    /// Proof: `ParachainSystem::DidSetValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    fn set_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `127`
        //  Estimated: `1612`
        // Minimum execution time: 146_810_106_000 picoseconds.
        Weight::from_parts(148_539_820_000, 1612)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Skipped::Metadata` (r:0 w:0)
    /// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `i` is `[0, 1000]`.
    fn set_storage(i: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_481_000 picoseconds.
        Weight::from_parts(2_574_000, 0)
            // Standard Error: 1_712
            .saturating_add(Weight::from_parts(647_187, 0).saturating_mul(i.into()))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
    }
    /// Storage: `Skipped::Metadata` (r:0 w:0)
    /// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `i` is `[0, 1000]`.
    fn kill_storage(i: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_379_000 picoseconds.
        Weight::from_parts(2_431_000, 0)
            // Standard Error: 672
            .saturating_add(Weight::from_parts(475_219, 0).saturating_mul(i.into()))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
    }
    /// Storage: `Skipped::Metadata` (r:0 w:0)
    /// Proof: `Skipped::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `p` is `[0, 1000]`.
    fn kill_prefix(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `89 + p * (69 ±0)`
        //  Estimated: `84 + p * (70 ±0)`
        // Minimum execution time: 4_536_000 picoseconds.
        Weight::from_parts(4_661_000, 84)
            // Standard Error: 700
            .saturating_add(Weight::from_parts(1_003_519, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
            .saturating_add(Weight::from_parts(0, 70).saturating_mul(p.into()))
    }
    /// Storage: `System::AuthorizedUpgrade` (r:0 w:1)
    /// Proof: `System::AuthorizedUpgrade` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
    fn authorize_upgrade() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 9_122_000 picoseconds.
        Weight::from_parts(10_037_000, 0)
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::AuthorizedUpgrade` (r:1 w:1)
    /// Proof: `System::AuthorizedUpgrade` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
    /// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
    /// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::UpgradeRestrictionSignal` (r:1 w:0)
    /// Proof: `ParachainSystem::UpgradeRestrictionSignal` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::PendingValidationCode` (r:1 w:1)
    /// Proof: `ParachainSystem::PendingValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
    /// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::NewValidationCode` (r:0 w:1)
    /// Proof: `ParachainSystem::NewValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `ParachainSystem::DidSetValidationCode` (r:0 w:1)
    /// Proof: `ParachainSystem::DidSetValidationCode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    fn apply_authorized_upgrade() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `149`
        //  Estimated: `1634`
        // Minimum execution time: 153_269_188_000 picoseconds.
        Weight::from_parts(156_752_282_000, 1634)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
}
