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
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.2.0
//! DATE: 2025-06-03, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a07377df7860`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-para-evm-node
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
// /data/benchmark/HEADER-APACHE2
// --output
// /data/benchmark/runtime/src/weights/pallet_proxy.rs
// --template
// /data/benchmark/scripts/templates/deploy-weight-template.hbs

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
        //  Measured:  `182 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 11_631_000 picoseconds.
        Weight::from_parts(12_770_462, 4310)
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
        //  Measured:  `434 + a * (56 ±0) + p * (25 ±0)`
        //  Estimated: `5302`
        // Minimum execution time: 32_834_000 picoseconds.
        Weight::from_parts(33_256_105, 5302)
            // Standard Error: 2_377
            .saturating_add(Weight::from_parts(119_644, 0).saturating_mul(a.into()))
            // Standard Error: 2_456
            .saturating_add(Weight::from_parts(26_648, 0).saturating_mul(p.into()))
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
        //  Measured:  `362 + a * (56 ±0)`
        //  Estimated: `5302`
        // Minimum execution time: 23_636_000 picoseconds.
        Weight::from_parts(23_030_983, 5302)
            // Standard Error: 3_335
            .saturating_add(Weight::from_parts(160_059, 0).saturating_mul(a.into()))
            // Standard Error: 3_446
            .saturating_add(Weight::from_parts(54_307, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn reject_announcement(a: u32, _p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `362 + a * (56 ±0)`
        //  Estimated: `5302`
        // Minimum execution time: 23_585_000 picoseconds.
        Weight::from_parts(24_611_530, 5302)
            // Standard Error: 1_429
            .saturating_add(Weight::from_parts(120_376, 0).saturating_mul(a.into()))
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
        //  Measured:  `378 + a * (56 ±0) + p * (25 ±0)`
        //  Estimated: `5302`
        // Minimum execution time: 29_900_000 picoseconds.
        Weight::from_parts(30_542_812, 5302)
            // Standard Error: 1_780
            .saturating_add(Weight::from_parts(124_929, 0).saturating_mul(a.into()))
            // Standard Error: 1_839
            .saturating_add(Weight::from_parts(15_597, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn add_proxy(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `182 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 20_002_000 picoseconds.
        Weight::from_parts(20_729_802, 4310)
            // Standard Error: 1_339
            .saturating_add(Weight::from_parts(41_361, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn remove_proxy(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `182 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 20_264_000 picoseconds.
        Weight::from_parts(21_231_396, 4310)
            // Standard Error: 1_158
            .saturating_add(Weight::from_parts(21_173, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn remove_proxies(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `182 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 18_246_000 picoseconds.
        Weight::from_parts(19_005_033, 4310)
            // Standard Error: 886
            .saturating_add(Weight::from_parts(19_667, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn create_pure(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `194`
        //  Estimated: `4310`
        // Minimum execution time: 21_145_000 picoseconds.
        Weight::from_parts(22_014_085, 4310)
            // Standard Error: 1_293
            .saturating_add(Weight::from_parts(27_143, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[0, 30]`.
    fn kill_pure(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `207 + p * (25 ±0)`
        //  Estimated: `4310`
        // Minimum execution time: 19_003_000 picoseconds.
        Weight::from_parts(19_743_253, 4310)
            // Standard Error: 1_149
            .saturating_add(Weight::from_parts(19_935, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
