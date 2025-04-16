import {describeSuite, expect, beforeAll, fetchCompiledContract } from "@moonwall/cli";
import { Wallet, ethers } from "ethers";
import zencashjs from 'zencashjs';

const ACC1_PK = '0xc3eed949eed88444607d36a46ba278061ac791e45f052014f1bd9552f5fa8f03';

describeSuite({
    id: 'ZC02',
    title: 'Test for SimpleProxy SDK Contract',
    foundationMethods: "dev",
    testCases: ({ it, context, log }) => {

        let provider;
        let wallet;
        let storageFactory;
        let simpleStorageContract;
        let simpleStorageAddress;
        let proxyFactory;
        let proxyContract;
        let proxyContractAddress;
        const REVERT_ERROR_MESSAGE_BEGIN = "transaction execution reverted";

        beforeAll(async () => {
            const { abi, bytecode } = fetchCompiledContract("SimpleStorage");
            const { abi: proxyAbi, bytecode: proxyBytecode } = fetchCompiledContract("SimpleProxy");
            provider = context.ethers().provider;
            wallet = new Wallet(ACC1_PK, provider);

            //deploy SimpleStorage
            storageFactory = new ethers.ContractFactory(
                abi as ethers.InterfaceAbi,
                bytecode,
                wallet
            );
            simpleStorageContract = await storageFactory.deploy();
            await simpleStorageContract.waitForDeployment();
            simpleStorageAddress = await simpleStorageContract.getAddress();
            log(`Simple Storage deployed to ${simpleStorageAddress}`);

            //deploy SimpleProxy
            proxyFactory = new ethers.ContractFactory(
                proxyAbi as ethers.InterfaceAbi, 
                proxyBytecode, 
                wallet
            );
            proxyContract = await proxyFactory.deploy({
                gasLimit: 1_000_000,
                gasPrice: 10_000_000_000,
            });
            await proxyContract.waitForDeployment();
            proxyContractAddress = await proxyContract.getAddress();
            log(`proxyContract deployed to ${proxyContractAddress}`);
        })
        
        it({
            id: '01',
            title: 'Test deployed Proxy contract',
            test: async () => {
                const value = 5;

                const setTxData = simpleStorageContract.interface.encodeFunctionData("set", [value]);
                const doCallTx = await proxyContract.doCall(simpleStorageAddress, 0, setTxData);                
                await doCallTx.wait();
                // check value was updated in the SimpleStorage 
                const newValue = await simpleStorageContract.get();
                await newValue.wait();
                expect(newValue).toEqual(value);

                //invoke set with doStaticCall
                let transactionSucceeded = false;
                const setTxDataForStaticCall = simpleStorageContract.interface.encodeFunctionData("set", [2*value]);
                try {
                    const doStaticCallTx = await proxyContract.doStaticCall(simpleStorageAddress, setTxDataForStaticCall, {gasLimit: 1000000});
                    await doStaticCallTx.wait();
                    transactionSucceeded = true;
                    log("Tx was supposed to revert but succeeded");
                } catch(error) {
                    expect(error.message.substring(0, REVERT_ERROR_MESSAGE_BEGIN.length)).toBe(REVERT_ERROR_MESSAGE_BEGIN);
                }
                expect(transactionSucceeded).toBe(false);

                //check value didn't changed
                const storageValue = await simpleStorageContract.get();
                await storageValue.wait();
                expect(storageValue).toEqual(value);
                
            }
        });

        // TODO: 'Testing proxy with Precomile (Zenclaim)',
    }
})
