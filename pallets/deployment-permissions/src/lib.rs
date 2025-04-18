#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused_crate_dependencies)]

extern crate alloc;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
pub mod runner;
#[cfg(test)]
mod tests;
pub mod weights;

pub use crate::weights::WeightInfo;
use frame_support::sp_runtime::DispatchError;
pub use pallet::*;
use sp_core::H160;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Deploy permission has been granted to an address.
        DeployPermissionGranted {
            /// The address to which deploy permission has been granted.
            address: H160,
        },
        /// Deploy permission has been revoked from an address.
        DeployPermissionRevoked {
            /// The address from which deploy permission has been revoked.
            address: H160,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Tried to grant deploy permission to an address already having it.
        AddressAlreadyHasDeployPermission,
        /// Tried to revoke deploy permission from an address not having it.
        AddressDoesNotHaveDeployPermission,
    }

    #[pallet::storage]
    pub type Deployers<T> = StorageMap<_, Blake2_128Concat, H160, (), OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::grant_deploy_permission())]
        pub fn grant_deploy_permission(origin: OriginFor<T>, address: H160) -> DispatchResult {
            ensure_root(origin)?;
            if !Deployers::<T>::contains_key(address) {
                Deployers::<T>::insert(address, ());
                Self::deposit_event(Event::<T>::DeployPermissionGranted { address });
                Ok(())
            } else {
                Err(Error::<T>::AddressAlreadyHasDeployPermission)?
            }
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::revoke_deploy_permission())]
        pub fn revoke_deploy_permission(origin: OriginFor<T>, address: H160) -> DispatchResult {
            ensure_root(origin)?;
            if Deployers::<T>::contains_key(address) {
                Deployers::<T>::remove(address);
                Self::deposit_event(Event::<T>::DeployPermissionRevoked { address });
                Ok(())
            } else {
                Err(Error::<T>::AddressDoesNotHaveDeployPermission)?
            }
        }
    }
}

impl<T: Config> EnsureCreateOrigin<T> for Pallet<T> {
    type Error = DispatchError;

    fn check_create_origin(address: &H160) -> Result<(), Self::Error> {
        if Deployers::<T>::contains_key(address) {
            Ok(())
        } else {
            Err(DispatchError::Other("Not allowed to deploy"))
        }
    }
}

pub trait EnsureCreateOrigin<T> {
    type Error: Into<DispatchError>;

    fn check_create_origin(address: &H160) -> Result<(), Self::Error>;
}
