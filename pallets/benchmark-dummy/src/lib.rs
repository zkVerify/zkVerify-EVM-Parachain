#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use frame_support::pallet_prelude::Weight;
pub use pallet::*;

/// A trait for pallet-specific weight information.
pub trait WeightInfo {
    /// The range of component `n` is `[0, 100]`.
    fn bench_heavy_contract_call(_n: u32) -> Weight;
}

#[frame_support::pallet]
pub mod pallet {
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_evm::Config {
        type WeightInfo: crate::WeightInfo;
    }
}

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights;
