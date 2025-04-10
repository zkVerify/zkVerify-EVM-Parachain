import {describeSuite, expect, beforeAll, fetchCompiledContract} from "@moonwall/cli";
import {ethers, Wallet} from "ethers";
import { ApiPromise } from "@polkadot/api";

const ACC1 =  '0x672bFCf89CFac1a32060C19419E8D598836ff088';
const ACC1_PK = '0xc3eed949eed88444607d36a46ba278061ac791e45f052014f1bd9552f5fa8f03';
const ACC2 = '0x186E534Dd9F445A5E61b40d31646826354137C51';
const ACC2_PK = '0x2858c71a3b9189220d5bc6c8cdfb253d1f90c5f6bda8ec262a005ec5a4cc0719';

describeSuite({
    id: "ZC01",
    title: "Zombie Custom Test Suite",
    foundationMethods: "zombie",
    testCases: ({ it, context, log }) => {
        let parachainApi: ApiPromise;

        beforeAll(async () => {
            parachainApi = context.polkadotJs("parachain");
        });

        it({
            id: "001",
            title: "Test of a simple EOA2EOA transaction inside the parachain",
            test: async function () {
                const acc1_before = (await parachainApi.query.system.account(ACC1)).data.free.toHuman();
                const acc2_before = (await parachainApi.query.system.account(ACC2)).data.free.toHuman();
                expect(acc1_before).to.be.eq("10,000,000,000,000,000,000");
                expect(acc2_before).to.be.eq("0");
                const signer = new Wallet(ACC1_PK, context.ethers().provider);

                await signer.sendTransaction({
                    to: ACC2,
                    value: 1000n,
                    nonce: await signer.getNonce(),
                });
                await context.waitBlock(1);
                //const acc1_after = (await parachainApi.query.system.account(ACC1)).data.free;
                const acc2_after = (await parachainApi.query.system.account(ACC2)).data.free.toHuman();
                //expect(acc1_before).to.be.eq(10_000_000_000_000_000_000n);
                expect(acc2_after).to.be.eq("1,000");
            },
        });

        it({
            id: "002",
            title: "Test of a simple smart contract deployment to the parachain",
            test: async function () {
                const provider = context.ethers().provider;
                const wallet = new Wallet(ACC1_PK, provider);
                log("Deploying contract from "+wallet.address);
                const { abi, bytecode } = fetchCompiledContract("SimpleStorage");

                const factory = new ethers.ContractFactory(
                    abi as ethers.InterfaceAbi,
                    bytecode,
                    wallet
                );
                log("Deploying..")
                const contract = await factory.deploy({
                    gasLimit: 1_000_000,
                    gasPrice: 10_000_000_000,
                    nonce: await context.ethers().getNonce() + 1,
                });

                log("Waiting deploy tx to be mined..")
                //this will wait until the deploy tx is mined
                let receipt = await contract.deploymentTransaction().wait();
                expect(receipt.status).to.be.eq(1);

                log("Tx mined in block ["+receipt.blockNumber+"] - Tx hash: "+receipt.hash);
                const contractAddress = await contract.getAddress();
                log("Contract deployed to address: ", contractAddress);


                //call set method on the contract
                console.log("Calling set method on the smart contract");
                const txSet = await contract.set(555);


                // Wait for the transaction to be mined...
                console.log("Waiting set tx to be mined..")
                receipt = await txSet.wait();
                const txHash = receipt?.hash;
                expect(receipt.status).to.be.eq(1);

                console.log("Set tx mined in block ["+receipt.blockNumber+"] - Tx hash: "+receipt.hash);

                //now check the set vaue has been stored/**/
                const value = await contract.get();
                expect(value).to.be.eq(555n);

                log("calling debug trace with callList tracer...");
                const result1 = await provider?.send("debug_traceTransaction",
                    [
                        txHash,
                        {
                           tracer: "callTracer"
                        }
                    ]
                );

                expect(result1.from).toEqual(wallet.address.toLowerCase());
                expect(result1.to).toEqual(contractAddress.toLowerCase());
                expect(result1.type).toEqual('CREATE');


                log("calling debug trace with op list (minimal) tracer...");
                const result2 = await provider?.send("debug_traceTransaction",
                    [
                        txHash,
                        {
                            disableMemory: true,
                            disableStack: true,
                            disableStorage: true,
                        }
                    ]
                );

                expect(result2.structLogs.length).toBeLessThan(160);
                expect(result2.returnValue).toEqual("0000000000000000000000000000000000000000000000000000000000000001");

            },
        });
    },
});
0
