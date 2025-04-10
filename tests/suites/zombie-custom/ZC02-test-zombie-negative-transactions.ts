import {describeSuite, expect, beforeAll } from "@moonwall/cli";
import {ethers, Wallet} from "ethers";

describeSuite({
    id: 'ZC02',
    title: 'Test of negative cases in a parachain',
    foundationMethods: 'zombie',
    testCases: async ({ it, context, log }) => {
        const standardGasLimit = BigInt(21000);
        const PK = "0xc3eed949eed88444607d36a46ba278061ac791e45f052014f1bd9552f5fa8f03";
        const receiver = "0x186E534Dd9F445A5E61b40d31646826354137C51";
        let provider;
        let wallet;
        let maxFeePerGas;

        beforeAll(async () => {
            provider = context.ethers().provider;
            wallet = new Wallet(PK, provider);
            const feeData = await provider.getFeeData();
            maxFeePerGas = feeData.maxFeePerGas;
        });

        it({
            id: "01",
            title: "Fails for reused nonce",
            test: async () => {
                // 1 - Send a transaction with a nonce already used.
                // send one nonce, then use it again
                console.log("Testing reused nonce...");
                const tx1 = await wallet.sendTransaction({
                    to: receiver,
                    value: ethers.parseEther("1.0"), 
                });
                const originalNonce = tx1.nonce;
                const receipt1 = await tx1.wait();
                expect(receipt1.status).toBe(1);

                let transactionSucceeded = false;
                try {
                    const tx2 = {
                        amount: ethers.parseEther("1.0"), 
                        to: receiver, 
                        nonce: originalNonce,
                    }
                    await wallet.sendTransaction({ 
                        ...tx2,
                        gasLimit: standardGasLimit
                    });
                    transactionSucceeded = true;
                    console.log("Transaction succeeded with resued nonce"); // should not print
                } catch (error) {
                    expect(error.code).toBe("NONCE_EXPIRED");
                }

                expect(transactionSucceeded).toBe(false);
            }
        });

        it({
            id: '02',
            title: 'Insufficient Gas',
            test: async () => {
                let transactionSucceeded = false;
                try {
                    const transaction = {
                        to: receiver,
                        value: ethers.parseEther("1.0"),
                    };
                    const estimatedGas = await provider.estimateGas(transaction);
                    await wallet.sendTransaction({
                        ...transaction,
                        gasLimit: estimatedGas / BigInt(2)
                    });
                    transactionSucceeded = true;
                } catch (error) {
                    expect(error?.error?.message).toBe('intrinsic gas too low');
                }
                expect(transactionSucceeded).toBe(false);
            }
        });

        it({
            id: '03',
            title: 'Insufficient balance',
            test: async () => {
                let transactionSucceeded = false;
                try {
                    const balance = await provider.getBalance(wallet.address);
                    await wallet.sendTransaction({
                        to: receiver,
                        value: balance * BigInt(2),
                        gasLimit: standardGasLimit,
                    });
                    transactionSucceeded = true;
                } catch (error) {
                    expect(error.code).toBe('INSUFFICIENT_FUNDS')
                }

                expect(transactionSucceeded).toBe(false);
            }
        });

        it({
            id: '04',
            title: 'Transaction tip > max fee per gas',
            test: async () => {
                let transactionSucceeded = false;
                try {
                    await wallet.sendTransaction({
                        to: receiver,
                        value: ethers.parseEther("1.0"),
                        maxPriorityFeePerGas: maxFeePerGas * BigInt(2),
                        gasLimit: standardGasLimit,
                    });
                    transactionSucceeded = true;
                } catch (error) {
                    // not sure of a way to verify specifically for this error message
                    expect(error?.shortMessage).toBe('priorityFee cannot be more than maxFee');
                }

                expect(transactionSucceeded).toBe(false);
            }
        });

        // 5 - Transaction with size greater than max size (in KB)
        // Commenting out the test below, as the tests here fail with 'intrinsic gas too low'
        // which does not reflect the transaction size
        /*
        console.log("Testing transaction size > max size...");
        const largeData = Array(1000 * 1024);
        largeData[0] = "0x";
        largeData.fill("FF", 1);
        try {
            await wallet.sendTransaction({
                to: receiver,
                data: largeData.join(""),
                value: ethers.parseEther("1"),
                gasLimit: standardGasLimit,
            });
            return ReturnCode.KO;
        } catch (error) {
            console.log(`Transaction failed with: ${error?.error?.message}`);
        }
        */

        it({
            id: '06',
            title: 'Gas greater than gas limit',
            test: async () => {
                let transactionSucceeded = false;
                try {
                    await wallet.sendTransaction({
                        to: receiver,
                        value: ethers.parseEther("0.1"),
                        gasLimit: BigInt('30000000'),
                    });
                    transactionSucceeded = true;
                } catch (error) {
                    log(`Transaction failed with: ${error?.error?.message}`);
                    expect(error?.error?.message.includes("gas")).toBe(true);
                }
                expect(transactionSucceeded).toBe(false);
            }
        })
    }
})
