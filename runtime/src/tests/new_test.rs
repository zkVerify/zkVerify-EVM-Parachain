mod mock_runtime;
use mock_runtime::*;

mod evm_benchmarks {
    use super::*;
    use frame_support::traits::Currency;
    use pallet_evm::{AddressMapping};
    use sp_core::{H160};
    use hex_literal::hex;

    type AccountIdOf<T> = <<T as pallet_evm::Config>::AccountProvider as pallet_evm::AccountProvider>::AccountId;
    type BalanceOf<T> = <<T as pallet_evm::Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

    fn create_funded_evm_account<T: pallet_evm::Config>(amount: BalanceOf<T>) -> (H160, AccountIdOf<T>) {
        let h160 = H160::repeat_byte(0x11);
        let account_id = <T as pallet_evm::Config>::AddressMapping::into_account_id(h160);
        <T as pallet_evm::Config>::Currency::deposit_creating(&account_id, amount);
        (h160, account_id)
    }

    #[test]
    fn bench_heavy_contract_call() {
        let mut ext = new_test_ext();
        ext.execute_with(|| {
            type T = crate::Runtime;
            use hex_literal::hex;
            use sp_core::{U256, H160};
            use pallet_evm::Pallet;

            // Fund an EVM account
            let amount: BalanceOf<T> = 1_000_000_000_000_000_000_000_000u128.into();
            let (source, substrate_account) = create_funded_evm_account::<T>(amount);

            // Contract bytecode
            let contract_bytecode: Vec<u8> = hex!(
                "6080604052348015600e575f5ffd5b506102308061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610034575f3560e01c80633fa4f245146100385780636584ad8b14610056575b5f5ffd5b610040610072565b60405161004d91906100dd565b60405180910390f35b610070600480360381019061006b9190610124565b610077565b005b5f5481565b805f54610084919061017c565b5f819055505f5f548260405160200161009e9291906101cf565b604051602081830303815290604052805190602001205f1c9050805f54185f819055505050565b5f819050919050565b6100d7816100c5565b82525050565b5f6020820190506100f05f8301846100ce565b92915050565b5f5ffd5b610103816100c5565b811461010d575f5ffd5b50565b5f8135905061011e816100fa565b92915050565b5f60208284031215610139576101386100f6565b5b5f61014684828501610110565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610186826100c5565b9150610191836100c5565b92508282019050808211156101a9576101a861014f565b5b92915050565b5f819050919050565b6101c96101c4826100c5565b6101af565b82525050565b5f6101da82856101b8565b6020820191506101ea82846101b8565b602082019150819050939250505056fea26469706673582212208e02abce628bcfccab523202b1013d6da271be717c0a9a2cd26b6e881aa76e1964736f6c634300081e0033"
            ).to_vec();

            let value = U256::zero();
            let gas_limit: u64 = 5_000_000;
            let max_fee_per_gas = U256::from(1_000_000_000u64);
            let max_priority_fee_per_gas = Some(U256::from(1_000_000_000u64));
            let nonce = Some(U256::zero());
            let access_list: Vec<(H160, Vec<sp_core::H256>)> = vec![];

            // Deploy contract
            let deploy_res = Pallet::<T>::create(
                frame_system::RawOrigin::Signed(substrate_account.clone()).into(),
                source,
                contract_bytecode.clone(),
                value,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                nonce,
                access_list.clone(),
            );

            assert!(deploy_res.is_ok(), "Contract deployment failed: {:?}", deploy_res);

            let contract_address = H160::from(hex!("d9145cce52d386f254917e481eb44e9943f39138"));

            // Function selector for doWork(uint256)
            let selector = [0x65, 0x84, 0xad, 0x8b];
            let mut arg = [0u8; 32];
            U256::from(12345u64).to_big_endian(&mut arg);
            let mut input: Vec<u8> = selector.to_vec();
            input.extend_from_slice(&arg);

            // Call contract multiple times and measure wall-clock time
            let n_calls = 100;
            let start = std::time::Instant::now();

            for i in 0..n_calls {
                U256::from(i).to_big_endian(&mut arg);
                input[4..].copy_from_slice(&arg);
                let res = Pallet::<T>::call(
                    frame_system::RawOrigin::Signed(substrate_account.clone()).into(),
                    source,
                    contract_address,
                    input.clone(),
                    value,
                    gas_limit,
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                    Some(U256::from(i + 1)), // increment nonce for each call
                    access_list.clone(),
                );

                assert!(res.is_ok(), "EVM call failed: {:?}", res);
            }
            let elapsed = start.elapsed();
            println!("Heavy contract {} calls executed in {:?}", n_calls, elapsed);
        });
    }

}
