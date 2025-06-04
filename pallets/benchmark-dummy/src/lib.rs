#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;
pub use weights::SubstrateWeight;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::WeightInfo;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_evm::Config {
        type WeightInfo: WeightInfo;
    }
}

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights;
