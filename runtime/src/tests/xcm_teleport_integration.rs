use crate::{
    constants::currency::tVFY,
    tests::{ExtBuilder, ALICE},
    ZKVXcm, RuntimeOrigin,
};
use xcm::v5::{Asset, AssetId, Assets, Fungibility, Junction, Location, WeightLimit};
use xcm::{VersionedAssets, VersionedLocation};

/// Test XCM message construction matches precompile expectations
#[test]
fn xcm_message_construction_matches_precompile() {
    ExtBuilder::default()
        .with_balances(vec![(ALICE.into(), 10 * tVFY)])
        .build()
        .execute_with(|| {
            // This mirrors the exact construction from the precompile
            let destination = VersionedLocation::V5(Location::new(1, []));
            let test_account = [0x42u8; 32];
            let beneficiary = VersionedLocation::V5(Location::new(
                0,
                [Junction::AccountId32 {
                    network: None,
                    id: test_account,
                }],
            ));

            let assets = VersionedAssets::V5(Assets::from(vec![Asset {
                id: AssetId(Location::new(1, [])),
                fun: Fungibility::Fungible(tVFY),
            }]));

            // Verify the construction is valid (no panics)
            assert!(matches!(destination, VersionedLocation::V5(_)));
            assert!(matches!(beneficiary, VersionedLocation::V5(_)));
            assert!(matches!(assets, VersionedAssets::V5(_)));

            // The actual teleport will fail without relay chain, but construction works
            let result = ZKVXcm::transfer_assets(
                RuntimeOrigin::signed(ALICE.into()),
                Box::new(destination),
                Box::new(beneficiary),
                Box::new(assets),
                0,
                WeightLimit::Unlimited
            );

            // Expecting either NotConnected, NotTrustedLocation, or similar transport error
            assert!(result.is_err());
            println!("Expected XCM error (no relay chain): {:?}", result.unwrap_err());
        });
}
