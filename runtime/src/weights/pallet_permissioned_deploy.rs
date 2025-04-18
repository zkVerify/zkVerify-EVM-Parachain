
//! Autogenerated weights for `pallet_permissioned_deploy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-04-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `DESKTOP-37J5UBI`, CPU: `Intel(R) Core(TM) Ultra 9 185H`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/release/zkv-para-evm-node
// benchmark
// pallet
// --pallet
// pallet-permissioned-deploy
// --extrinsic
// 
// --output
// runtime/src/weights/pallet_permissioned_deploy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_permissioned_deploy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_permissioned_deploy::WeightInfo for WeightInfo<T> {
	/// Storage: `PermissionedDeploy::Deployers` (r:1 w:1)
	/// Proof: `PermissionedDeploy::Deployers` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn grant_deploy_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `3501`
		// Minimum execution time: 9_910_000 picoseconds.
		Weight::from_parts(10_511_000, 0)
			.saturating_add(Weight::from_parts(0, 3501))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PermissionedDeploy::Deployers` (r:1 w:1)
	/// Proof: `PermissionedDeploy::Deployers` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn revoke_deploy_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `175`
		//  Estimated: `3501`
		// Minimum execution time: 11_747_000 picoseconds.
		Weight::from_parts(12_099_000, 0)
			.saturating_add(Weight::from_parts(0, 3501))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
