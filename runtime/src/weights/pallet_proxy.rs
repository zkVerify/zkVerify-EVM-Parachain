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

//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-04-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// /home/mdamico/devel/zkVerify-EVM-Parachain/target/release/zkv-para-evm-node
// benchmark
// pallet
// --genesis-builder=spec
// --pallet
// pallet-proxy
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
// /home/mdamico/devel/zkVerify-EVM-Parachain/runtime/src/weights/pallet_proxy.rs
// --template
// /home/mdamico/devel/zkVerify-EVM-Parachain/scripts/templates/deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_proxy` using the zkVerify node and recommended hardware.
pub struct ZKVEvmWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_proxy::WeightInfo for ZKVEvmWeight<T> {
    /// Storage: `Proxy::Proxies` (r:1 w:0)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn proxy(_p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `149 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 19_104_000 picoseconds.
        Weight::from_parts(31_895_325, 4310)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:0)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn proxy_announced(a: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `401 + a * (56 ±0) + p * (25 ±0)`
        //  Estimated: `5302`
        // Minimum execution time: 48_026_000 picoseconds.
        Weight::from_parts(51_457_337, 5302)
            // Standard Error: 12_651
            .saturating_add(Weight::from_parts(284_808, 0).saturating_mul(a.into()))
            // Standard Error: 13_071
            .saturating_add(Weight::from_parts(49_262, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn remove_announcement(a: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `329 + a * (56 ±0)`
        //  Estimated: `5302`
        // Minimum execution time: 35_054_000 picoseconds.
        Weight::from_parts(36_088_755, 5302)
            // Standard Error: 7_979
            .saturating_add(Weight::from_parts(224_144, 0).saturating_mul(a.into()))
            // Standard Error: 8_244
            .saturating_add(Weight::from_parts(45_235, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn reject_announcement(a: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `329 + a * (56 ±0)`
        //  Estimated: `5302`
        // Minimum execution time: 34_427_000 picoseconds.
        Weight::from_parts(37_229_541, 5302)
            // Standard Error: 7_344
            .saturating_add(Weight::from_parts(190_964, 0).saturating_mul(a.into()))
            // Standard Error: 7_588
            .saturating_add(Weight::from_parts(6_430, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:0)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn announce(a: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `345 + a * (56 ±0) + p * (25 ±0)`
        //  Estimated: `5302`
        // Minimum execution time: 44_676_000 picoseconds.
        Weight::from_parts(46_540_613, 5302)
            // Standard Error: 7_501
            .saturating_add(Weight::from_parts(204_892, 0).saturating_mul(a.into()))
            // Standard Error: 7_750
            .saturating_add(Weight::from_parts(40_803, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn add_proxy(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `149 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 33_213_000 picoseconds.
        Weight::from_parts(35_323_883, 4310)
            // Standard Error: 7_074
            .saturating_add(Weight::from_parts(22_756, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn remove_proxy(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `149 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 33_992_000 picoseconds.
        Weight::from_parts(35_585_440, 4310)
            // Standard Error: 6_538
            .saturating_add(Weight::from_parts(78_887, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn remove_proxies(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `149 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 30_588_000 picoseconds.
        Weight::from_parts(33_142_934, 4310)
            // Standard Error: 6_754
            .saturating_add(Weight::from_parts(1_063, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn create_pure(_p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `161`
        //  Estimated: `4310`
        // Minimum execution time: 35_015_000 picoseconds.
        Weight::from_parts(38_560_817, 4310)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[0, 30]`.
    fn kill_pure(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `174 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 30_934_000 picoseconds.
        Weight::from_parts(33_544_893, 4310)
            // Standard Error: 17_423
            .saturating_add(Weight::from_parts(40_442, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
