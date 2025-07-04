// Copyright 2025, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::*;
use frame_benchmarking::v2::*;

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as Template;
    use frame_system::RawOrigin;

    #[benchmark]
    fn grant_deploy_permission() {
        let address = H160::repeat_byte(42);

        #[extrinsic_call]
        grant_deploy_permission(RawOrigin::Root, address);

        assert!(Deployers::<T>::get(address).is_some());
    }

    #[benchmark]
    fn revoke_deploy_permission() {
        let address = H160::repeat_byte(42);
        Deployers::<T>::insert(address, ());

        #[extrinsic_call]
        revoke_deploy_permission(RawOrigin::Root, address);

        assert!(Deployers::<T>::get(address).is_none());
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
