import { describeSuite, expect, fetchCompiledContract, beforeAll } from "@moonwall/cli";
import { Wallet, ContractFactory, InterfaceAbi } from "ethers";

describeSuite({
    id: 'Z03',
    title: 'Test of eth debug methods',
    foundationMethods: 'zombie',
    testCases: ({ it, context, log }) => {
        const PK = "4ddbe44489f20fbc179f758d97ae1969319774211496c2d394facc44ff0e0e65";
        let provider;
        let wallet;

        beforeAll(() => {
            provider = context.ethers().provider;
            wallet = new Wallet(PK, provider);
        });

        it({
            id: '01',
            title: 'Test EOA to EOA transactions',
            test: async () => {
                const to_address = "0xA15a1AAb9B797CeAeb4E656231243F26Ec71C78A";
                const amount_val_1 = 1000;
                const amount_val_2 = 2000;
                const nonce = await wallet.getNonce();

                const tx_eoa_1 = await wallet.sendTransaction({
                    from: wallet.address,
                    to: to_address,
                    value: amount_val_1,
                    nonce: nonce
                  });
                  log("TX1="+tx_eoa_1.hash);
                  log("nonce="+tx_eoa_1.nonce);
              
                const tx_eoa_2 = await wallet.sendTransaction({
                    from: wallet.address,
                    to: to_address,
                    value: amount_val_2,
                    nonce: nonce+1
                });
                log("TX2="+tx_eoa_2.hash);
                log("nonce="+tx_eoa_2.nonce);
            
                log("Waiting tx to be mined..")    
                //This will wait until the transaction is mined
                const receipt_1 = await tx_eoa_1.wait();
                const receipt_2 = await tx_eoa_2.wait();

                expect(receipt_1?.status).toBe(1);
                expect(receipt_2?.status).toBe(1);
                // Txes should belong to the same block
                expect(receipt_1?.blockHash).toBe(receipt_2?.blockHash);

                const blockNumber = "0x"+receipt_1?.blockNumber.toString(16);
                const blockHash = receipt_1?.blockHash;
                log("Block number: "+blockNumber);
                log("Block hash: "+blockHash);
                const receiptGasUsed = '0x'+receipt_1?.gasUsed.toString(16);

                log("---> calling debug trace with callList tracer...");
                const result = await provider?.send("debug_traceTransaction",
                    [
                        tx_eoa_2.hash,
                        {
                            tracer: "callTracer"
                        }
                    ]
                );
                log(result);  // Tracing output of the transaction
                const resultGasUsed = result.gasUsed;
                expect(receiptGasUsed).toBe(resultGasUsed);
                expect(result.from).toBe(wallet.address.toLowerCase())
                expect(result.to).toBe(to_address.toLowerCase())
                const val = BigInt(result.value);
                expect(BigInt(amount_val_2)).toBe(val);

                log("---> calling debug trace block by hash with callList tracer...");
                const result1 = await provider?.send("debug_traceBlockByHash",
                    [
                        blockHash,
                        {
                           tracer: "callTracer"
                        }
                    ]
                );
                log(result1);  // Tracing output of the transaction
                expect(result1.length).toBe(2);
                
                log("---> calling debug trace block by number with callList tracer...");
                const result2 = await provider?.send("debug_traceBlockByNumber",
                    [
                        blockNumber,
                        {
                           tracer: "callTracer"
                        }
                    ]
                );
                log(result2);  // Tracing output of the transaction
                expect(JSON.stringify(result1)).toBe(JSON.stringify(result2));

                log("---> calling debug trace block with default tracer... NOT SUPPORTED as of now, expecting an error");
                try {
                    const expectError = await provider?.send("debug_traceBlockByNumber",
                        [
                            blockHash,
                            {
                                enableMemory: false,
                                disableStack: true,
                                disableStorage: true,
                            }
                        ]
                    );
                    expect(false, "Expected error, should not get here").toBe(true);
                    log(expectError);  // Tracing output of the transaction
                } catch (error) {
                    log("Expected error was thrown")
                }
                
            }
        });

        it({
            id: '02',
            title: 'Test a Solidity Contract',
            test: async () => {
                const { abi, bytecode } = fetchCompiledContract("TestERC20");
                const factory = new ContractFactory(
                    abi as InterfaceAbi,
                    bytecode, 
                    wallet
                );

                const contract = await factory.deploy();
                const deploymentTx = contract.deploymentTransaction();
                const contract_address = await contract.getAddress();
                const receipt = await deploymentTx?.wait();
                expect(receipt?.status).toBe(1);
                log("Contract deployed to address: ", contract_address);    
                log("gasUsed in receipt ["+receipt?.gasUsed+"]");
                const txHash = receipt?.hash;

                /*
                NOTE: original script eth_debug_test_sol_contract.js called this RPC method
                but did not check for anything, so skipping here in the moonwall tests
                log("calling debug trace with default tracer...");
                const result = await provider?.send("debug_traceTransaction",
                    [
                        txHash
                    ]
                );
                */

                log("calling debug trace with callList tracer...");
                const result = await provider?.send("debug_traceTransaction",
                    [
                        txHash,
                        {
                        tracer: "callTracer"
                        }
                    ]
                );
                // Currently not clear in this case where does the gasUsed value comes from!
                log(result);  
                const gas = result.gas
                const gasUsed = result.gasUsed
                log("gas in result = "+ gas + ", gasUsed in result = "+ gasUsed);
                expect(result.from).toBe(wallet.address.toLowerCase());
                expect(result.to).toBe(contract_address.toLowerCase());
                expect(result.type).toBe('CREATE');
                
                // TODO - consider making this a separate test (new it())
                log("==============================================================")
                log("Calling 'transfer' method on the smart contract");
                const other_address = "0x7c8ce92d470769176207c42c6032fef5b0848065";
                const txSet = await contract.transfer(other_address, 99);
                log("Waiting tx with contract call to be mined..");
                const transferReceipt = await txSet.wait();
                expect(transferReceipt.status).toBe(1);
                const transferTxHash = transferReceipt.hash;
                const transferBlockNumber = "0x"+transferReceipt.blockNumber.toString(16);
                log("tx mined in block ["+transferBlockNumber+"] - Tx hash: "+transferTxHash);

                log("---> calling debug trace with callList tracer...");
                const debugResult = await provider?.send("debug_traceTransaction",
                    [
                        transferTxHash,
                        {
                            tracer: "callTracer"
                        }
                    ]
                );
                expect(debugResult.from).toBe(wallet.address.toLowerCase());
                expect(debugResult.to).toBe(contract_address.toLowerCase());
                expect(debugResult.type).toBe('CALL');
                expect(debugResult.output).toBe('0x0000000000000000000000000000000000000000000000000000000000000001');

                // Minimal tracer
                log("---> calling debug trace with op list (minimal) tracer...");
                const debugResult2 = await provider?.send("debug_traceTransaction",
                    [
                        transferTxHash,
                        {
                            disableMemory: true,
                            disableStack: true,
                            disableStorage: true,
                        }
                    ]
                );
                log(debugResult2);  // Tracing output of the transaction
                expect(debugResult2.structLogs.length).toBeGreaterThan(160);
                // boolean result, true
                expect(debugResult2.returnValue).toBe("0000000000000000000000000000000000000000000000000000000000000001");
            }
        })
    }
})
