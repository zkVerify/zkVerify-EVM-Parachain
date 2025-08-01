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

//! Autogenerated weights for `pallet_message_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.2.0
//! DATE: 2025-06-03, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `2142d8558447`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-para-evm-node
// benchmark
// pallet
// --genesis-builder=spec
// --pallet
// pallet-message-queue
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
// /data/benchmark/runtime/src/weights/pallet_message_queue.rs
// --template
// /data/benchmark/scripts/templates/deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_message_queue` using the zkVerify node and recommended hardware.
pub struct ZKVEvmWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_message_queue::WeightInfo for ZKVEvmWeight<T> {
    /// Storage: `MessageQueue::ServiceHead` (r:1 w:0)
    /// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
    /// Storage: `MessageQueue::BookStateFor` (r:2 w:2)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    fn ready_ring_knit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `223`
        //  Estimated: `6044`
        // Minimum execution time: 16_758_000 picoseconds.
        Weight::from_parts(17_186_000, 6044)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `MessageQueue::BookStateFor` (r:2 w:2)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
    /// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
    fn ready_ring_unknit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `218`
        //  Estimated: `6044`
        // Minimum execution time: 14_946_000 picoseconds.
        Weight::from_parts(15_658_000, 6044)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    fn service_queue_base() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `6`
        //  Estimated: `3517`
        // Minimum execution time: 4_918_000 picoseconds.
        Weight::from_parts(5_133_000, 3517)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `MessageQueue::Pages` (r:1 w:1)
    /// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
    fn service_page_base_completion() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `72`
        //  Estimated: `69050`
        // Minimum execution time: 7_548_000 picoseconds.
        Weight::from_parts(8_081_000, 69050)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `MessageQueue::Pages` (r:1 w:1)
    /// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
    fn service_page_base_no_completion() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `72`
        //  Estimated: `69050`
        // Minimum execution time: 7_624_000 picoseconds.
        Weight::from_parts(8_028_000, 69050)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `MessageQueue::BookStateFor` (r:0 w:1)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `MessageQueue::Pages` (r:0 w:1)
    /// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
    fn service_page_item() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 200_633_000 picoseconds.
        Weight::from_parts(203_025_000, 0)
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
    /// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
    /// Storage: `MessageQueue::BookStateFor` (r:1 w:0)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    fn bump_service_head() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `171`
        //  Estimated: `3517`
        // Minimum execution time: 8_890_000 picoseconds.
        Weight::from_parts(9_262_000, 3517)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `MessageQueue::Pages` (r:1 w:1)
    /// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
    fn reap_page() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `65667`
        //  Estimated: `69050`
        // Minimum execution time: 62_298_000 picoseconds.
        Weight::from_parts(63_385_000, 69050)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `MessageQueue::Pages` (r:1 w:1)
    /// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
    fn execute_overweight_page_removed() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `65667`
        //  Estimated: `69050`
        // Minimum execution time: 76_614_000 picoseconds.
        Weight::from_parts(78_183_000, 69050)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `MessageQueue::Pages` (r:1 w:1)
    /// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
    fn execute_overweight_page_updated() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `65667`
        //  Estimated: `69050`
        // Minimum execution time: 108_508_000 picoseconds.
        Weight::from_parts(113_557_000, 69050)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
}
