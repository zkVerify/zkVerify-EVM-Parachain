
//! Autogenerated weights for `pallet_sudo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-15-118`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/zkv-para-evm-node
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=benchmarking/results/results-pallet_sudo.json
// --pallet=pallet_sudo
// --chain=dev
// --output=benchmarking/new-benchmarks/pallet_sudo.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_sudo`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_sudo::WeightInfo for WeightInfo<T> {
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86`
		//  Estimated: `1505`
		// Minimum execution time: 12_120_000 picoseconds.
		Weight::from_parts(12_541_000, 0)
			.saturating_add(Weight::from_parts(0, 1505))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn sudo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86`
		//  Estimated: `1505`
		// Minimum execution time: 13_348_000 picoseconds.
		Weight::from_parts(13_701_000, 0)
			.saturating_add(Weight::from_parts(0, 1505))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn sudo_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86`
		//  Estimated: `1505`
		// Minimum execution time: 13_530_000 picoseconds.
		Weight::from_parts(13_906_000, 0)
			.saturating_add(Weight::from_parts(0, 1505))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn remove_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86`
		//  Estimated: `1505`
		// Minimum execution time: 11_222_000 picoseconds.
		Weight::from_parts(11_443_000, 0)
			.saturating_add(Weight::from_parts(0, 1505))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
