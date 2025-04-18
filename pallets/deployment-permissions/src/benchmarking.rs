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
