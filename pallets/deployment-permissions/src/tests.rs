use super::*;
use crate::mock::*;
use frame_support::{assert_err, assert_noop, assert_ok};

mod grant_deploy_permission {
    use super::*;

    #[test]
    fn sets_storage_key() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let address = H160::repeat_byte(42);
            assert!(!Deployers::<Test>::contains_key(address));
            assert_ok!(PalletDeployPermissions::grant_deploy_permission(
                RuntimeOrigin::root(),
                address
            ));
            assert!(Deployers::<Test>::contains_key(address));
        });
    }

    #[test]
    fn emits_correct_event() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let address = H160::repeat_byte(42);
            assert_ok!(PalletDeployPermissions::grant_deploy_permission(
                RuntimeOrigin::root(),
                address
            ));
            System::assert_last_event(Event::DeployPermissionGranted { address }.into());
        });
    }

    #[test]
    fn must_be_invoked_by_root() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let caller: <Test as frame_system::Config>::AccountId = 1;
            let address = H160::repeat_byte(42);
            assert_noop!(
                PalletDeployPermissions::grant_deploy_permission(
                    RuntimeOrigin::signed(caller),
                    address
                ),
                DispatchError::BadOrigin
            );
        })
    }

    #[test]
    fn errors_if_address_already_has_deploy_permission() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let address = H160::repeat_byte(42);
            Deployers::<Test>::insert(address, ());
            assert_noop!(
                PalletDeployPermissions::grant_deploy_permission(RuntimeOrigin::root(), address),
                Error::<Test>::AddressAlreadyHasDeployPermission
            );
        })
    }
}

mod revoke_deploy_permission {
    use super::*;

    #[test]
    fn clears_storage_key() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let address = H160::repeat_byte(42);
            Deployers::<Test>::insert(address, ());
            assert_ok!(PalletDeployPermissions::revoke_deploy_permission(
                RuntimeOrigin::root(),
                address
            ));
            assert!(!Deployers::<Test>::contains_key(address));
        });
    }

    #[test]
    fn emits_correct_event() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let address = H160::repeat_byte(42);
            Deployers::<Test>::insert(address, ());
            assert_ok!(PalletDeployPermissions::revoke_deploy_permission(
                RuntimeOrigin::root(),
                address
            ));
            System::assert_last_event(Event::DeployPermissionRevoked { address }.into());
        });
    }

    #[test]
    fn must_be_invoked_by_root() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let caller: <Test as frame_system::Config>::AccountId = 1;
            let address = H160::repeat_byte(42);
            Deployers::<Test>::insert(address, ());
            assert_noop!(
                PalletDeployPermissions::revoke_deploy_permission(
                    RuntimeOrigin::signed(caller),
                    address
                ),
                DispatchError::BadOrigin
            );
        })
    }

    #[test]
    fn errors_if_address_does_not_have_deploy_permission() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let address = H160::repeat_byte(42);
            assert_noop!(
                PalletDeployPermissions::revoke_deploy_permission(RuntimeOrigin::root(), address),
                Error::<Test>::AddressDoesNotHaveDeployPermission
            );
        })
    }
}

mod check_create_origin {
    use super::*;

    #[test]
    fn returns_ok_for_whitelisted_address() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let address = H160::repeat_byte(42);
            Deployers::<Test>::insert(address, ());
            assert_ok!(<Pallet<Test> as EnsureCreateOrigin<Test>>::check_create_origin(&address));
        })
    }

    #[test]
    fn returns_error_for_non_whitelisted_address() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            let address = H160::repeat_byte(42);
            assert_err!(
                <Pallet<Test> as EnsureCreateOrigin<Test>>::check_create_origin(&address),
                DispatchError::Other("Not allowed to deploy")
            );
        })
    }
}
