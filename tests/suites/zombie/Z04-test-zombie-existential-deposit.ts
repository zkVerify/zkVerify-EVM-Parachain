import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ethers, Wallet } from "ethers";
import { ALITH_PRIVATE_KEY, GERALD_PRIVATE_KEY, GERALD_ADDRESS } from "@moonwall/util"

describeSuite({
    id: 'Z04',
    title: 'Test of existential deposits',
    foundationMethods: 'zombie',
    testCases: async ({ it, context, log }) => {
        let provider;
        let walletAlith;
        let walletGerald;

        beforeAll(() => {
            provider = context.ethers().provider;
            walletAlith = new Wallet(ALITH_PRIVATE_KEY, provider);
            walletGerald = new Wallet(GERALD_PRIVATE_KEY, provider);
        });

        it({
            id: '01',
            title: "Test existential deposit",
            test: async () => {
                const receiver = "0x0000000000000000000000000000000000051095"; //random address to receive tx
                const MINIMAL_AMOUNT = 1n;

                //feed Gerald balance with some money
                log("filling sender balance" );
                const feedTx = await walletAlith.sendTransaction({value: ethers.parseEther("1.0"), to: GERALD_ADDRESS, type: 2});
                const receiptFeed = await feedTx.wait();
                expect(receiptFeed?.status).toBe(1);

                const senderBalance = await provider?.getBalance(GERALD_ADDRESS);
                log("wallet sender address: " + GERALD_ADDRESS);
                log("wallet sender balance: " + senderBalance);
        
                const nonce_0 = await walletGerald.getNonce();
        
                const minimalTx = await walletGerald.sendTransaction({value: MINIMAL_AMOUNT, to: receiver, type: 2});
                const receipt = await minimalTx.wait();
                expect(receipt?.status).toBe(1);
        
                const receiver_minimal_amount_balance = await provider?.getBalance(receiver);
                expect(receiver_minimal_amount_balance).toBe(MINIMAL_AMOUNT);
        
                const nonce_1 = await walletGerald.getNonce();
                expect(nonce_1).toBe(Number(nonce_0) + 1);
        
                const balance = BigInt((await provider.getBalance(GERALD_ADDRESS)));
                const feeData = await provider?.getFeeData();
                const gasPrice = BigInt(feeData?.maxFeePerGas ?? 0);
                log(`computed gasPrice in the next TX: ${gasPrice}`);
                const gasLimit = BigInt(21000); // Standard gas limit for a simple ETH transfer
                const totalGasFee = gasPrice*gasLimit;
                log(`This is the fee we ought to pay in the next TX: ${totalGasFee}`);
                const amountToSend = balance - totalGasFee;
                log(`This is the amount to send in the next TX: ${amountToSend}`);
        
                // Send the transaction for cleaning up all the available sender balance
                // -------------------------
                // We chose to send a legacy Transaction, where gasPrice is directly specified
                log("calling sendTransaction...");
                const txObj = await walletGerald.sendTransaction({
                    to: receiver,
                    // For EIP-1559 tx
                    //maxFeePerGas: feeData.maxFeePerGas
                    //maxPriorityFeePerGas: feeData.maxFeePerGas, // miner tip, set to the max value (see comments above)
                    gasPrice: gasPrice,
                    value: amountToSend,
                    type: 0 // Type 0 = legacy, 2=eip1559 
                });
                log("tx: " + txObj.hash);
                const sendReceipt = await txObj.wait();
                expect(sendReceipt?.status).toBe(1);
        
                const newBalance = BigInt((await provider?.getBalance(walletGerald.address)));
                log("final sender balance: " + newBalance.toString());
                expect(newBalance).toBe(0n);
            }
        })

    }
})
