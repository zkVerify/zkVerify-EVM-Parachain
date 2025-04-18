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

//! Substrate EVM tracing.
//!
//! The purpose of this crate is enable tracing the EVM opcode execution and will be used by
//! both Dapp developers - to get a granular view on their transactions - and indexers to access
//! the EVM callstack (internal transactions).
//!
//! Proxies EVM messages to the host functions.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod tracer {
    use evm::tracing::{using as evm_using, EventListener as EvmListener};
    use evm_gasometer::tracing::{using as gasometer_using, EventListener as GasometerListener};
    use evm_runtime::tracing::{using as runtime_using, EventListener as RuntimeListener};
    use evm_tracing_events::{EvmEvent, GasometerEvent, RuntimeEvent, StepEventFilter};
    use parity_scale_codec::Encode;
    use sp_std::{cell::RefCell, rc::Rc};

    struct ListenerProxy<T>(pub Rc<RefCell<T>>);
    impl<T: GasometerListener> GasometerListener for ListenerProxy<T> {
        fn event(&mut self, event: evm_gasometer::tracing::Event) {
            self.0.borrow_mut().event(event);
        }
    }

    impl<T: RuntimeListener> RuntimeListener for ListenerProxy<T> {
        fn event(&mut self, event: evm_runtime::tracing::Event) {
            self.0.borrow_mut().event(event);
        }
    }

    impl<T: EvmListener> EvmListener for ListenerProxy<T> {
        fn event(&mut self, event: evm::tracing::Event) {
            self.0.borrow_mut().event(event);
        }
    }

    pub struct EvmTracer {
        step_event_filter: StepEventFilter,
    }

    impl EvmTracer {
        pub fn new() -> Self {
            Self {
                step_event_filter: zkv_para_evm_primitives_ext::zkv_para_evm_ext::step_event_filter(
                ),
            }
        }

        /// Setup event listeners and execute provided closure.
        ///
        /// Consume the tracer and return it alongside the return value of
        /// the closure.
        pub fn trace<R, F: FnOnce() -> R>(self, f: F) {
            let wrapped = Rc::new(RefCell::new(self));

            let mut gasometer = ListenerProxy(Rc::clone(&wrapped));
            let mut runtime = ListenerProxy(Rc::clone(&wrapped));
            let mut evm = ListenerProxy(Rc::clone(&wrapped));

            // Each line wraps the previous `f` into a `using` call.
            // Listening to new events results in adding one new line.
            // Order is irrelevant when registering listeners.
            let f = || runtime_using(&mut runtime, f);
            let f = || gasometer_using(&mut gasometer, f);
            let f = || evm_using(&mut evm, f);
            f();
        }

        pub fn emit_new() {
            zkv_para_evm_primitives_ext::zkv_para_evm_ext::call_list_new();
        }
    }

    impl EvmListener for EvmTracer {
        /// Proxies `evm::tracing::Event` to the host.
        fn event(&mut self, event: evm::tracing::Event) {
            let event: EvmEvent = event.into();
            let message = event.encode();
            zkv_para_evm_primitives_ext::zkv_para_evm_ext::evm_event(message);
        }
    }

    impl GasometerListener for EvmTracer {
        /// Proxies `evm_gasometer::tracing::Event` to the host.
        fn event(&mut self, event: evm_gasometer::tracing::Event) {
            let event: GasometerEvent = event.into();
            let message = event.encode();
            zkv_para_evm_primitives_ext::zkv_para_evm_ext::gasometer_event(message);
        }
    }

    impl RuntimeListener for EvmTracer {
        /// Proxies `evm_runtime::tracing::Event` to the host.
        fn event(&mut self, event: evm_runtime::tracing::Event) {
            let event = RuntimeEvent::from_evm_event(event, self.step_event_filter);
            let message = event.encode();
            zkv_para_evm_primitives_ext::zkv_para_evm_ext::runtime_event(message);
        }
    }
}
