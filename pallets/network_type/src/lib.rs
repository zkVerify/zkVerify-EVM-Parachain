// Copyright 2025, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use serde::{Deserialize, Serialize};

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

    #[derive(
        Debug,
        Encode,
        Decode,
        TypeInfo,
        Default,
        Copy,
        Clone,
        Serialize,
        Deserialize,
        MaxEncodedLen,
        PartialEq,
    )]
    pub enum NetworkTypeEnum {
        #[default]
        TestNet = 1,
        MainNet = 2,
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type NT: Get<NetworkTypeEnum>;
    }

    impl<T: Config> Get<NetworkTypeEnum> for Pallet<T> {
        fn get() -> NetworkTypeEnum {
            <StoredNetworkType<T>>::get()
        }
    }

    /// The Network Type
    #[pallet::storage]
    pub type StoredNetworkType<T> = StorageValue<_, NetworkTypeEnum, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T> {
        pub value: NetworkTypeEnum,
        #[serde(skip)]
        pub _marker: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            StoredNetworkType::<T>::put(self.value);
        }
    }
}
