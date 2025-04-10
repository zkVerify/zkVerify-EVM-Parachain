// Copyright 2025, Horizen Labs, Inc.
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

//! Environmental-aware externalities for EVM tracing in Wasm runtime. This enables
//! capturing the - potentially large - trace output data in the host and keep
//! a low memory footprint in `--execution=wasm`.
//!
//! - The original trace Runtime Api call is wrapped `using` environmental (thread local).
//! - Arguments are scale-encoded known types in the host.
//! - Host functions will decode the input and emit an event `with` environmental.

#![cfg_attr(not(feature = "std"), no_std)]
#[allow(unused_imports)]
use evm_tracing_events::{Event, EvmEvent, GasometerEvent, RuntimeEvent, StepEventFilter};
#[allow(unused_imports)]
use parity_scale_codec::Decode;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

#[runtime_interface]
pub trait ZkvParaEvmExt {
    fn raw_step(&mut self, _data: Vec<u8>) {}

    fn raw_gas(&mut self, _data: Vec<u8>) {}

    fn raw_return_value(&mut self, _data: Vec<u8>) {}

    fn call_list_entry(&mut self, _index: u32, _value: Vec<u8>) {}

    fn call_list_new(&mut self) {}

    // New design, proxy events.
    /// An `Evm` event proxied by the runtime to this host function.
    /// evm -> runtime -> host.
    fn evm_event(&mut self, event: Vec<u8>) {
        if let Ok(event) = EvmEvent::decode(&mut &event[..]) {
            Event::Evm(event).emit();
        }
    }

    /// A `Gasometer` event proxied by the runtime to this host function.
    /// evm_gasometer -> runtime -> host.
    fn gasometer_event(&mut self, event: Vec<u8>) {
        if let Ok(event) = GasometerEvent::decode(&mut &event[..]) {
            Event::Gasometer(event).emit();
        }
    }

    /// A `Runtime` event proxied by the runtime to this host function.
    /// evm_runtime -> runtime -> host.
    fn runtime_event(&mut self, event: Vec<u8>) {
        if let Ok(event) = RuntimeEvent::decode(&mut &event[..]) {
            Event::Runtime(event).emit();
        }
    }

    /// Allow the tracing module in the runtime to know how to filter Step event
    /// content, as cloning the entire data is expensive and most of the time
    /// not necessary.
    fn step_event_filter(&self) -> StepEventFilter {
        evm_tracing_events::step_event_filter().unwrap_or_default()
    }

    /// An event to create a new CallList (currently a new transaction when tracing a block).
    #[version(2)]
    fn call_list_new(&mut self) {
        Event::CallListNew().emit();
    }
}
