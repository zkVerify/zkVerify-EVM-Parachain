import { describeSuite, expect } from "@moonwall/cli";
import { ALITH_PRIVATE_KEY, ALITH_ADDRESS, BALTATHAR_ADDRESS } from "@moonwall/util"
import { Wallet } from "ethers";

describeSuite({
    id: 'Z05',
    title: 'Test EIP-1559 and Legacy transactions',
    foundationMethods: 'zombie',
    testCases: async ({ context, it }) => {
        const AMOUNT = 10000;
        const EXP_GAS = 21000n;

        let alithBalance;
        let baltatharBalance;
        
        it({
            id: '01',
            title: 'Test of a legacy EOA2EOA transaction',
            test: async function () {
                const provider = context.ethers().provider;
                const alithWallet = new Wallet(ALITH_PRIVATE_KEY, provider);
                if (provider) {
                    alithBalance = await provider.getBalance(ALITH_ADDRESS);
                    baltatharBalance = await provider.getBalance(BALTATHAR_ADDRESS);
        
                    const legacyTx = await alithWallet.sendTransaction({value: AMOUNT, to: BALTATHAR_ADDRESS, type: 0}); //type 0 is legacy tx
                    await legacyTx.wait();
                    const legacyReceipt = await provider.getTransactionReceipt(legacyTx.hash);
                    if (legacyReceipt) {
                        expect(legacyReceipt?.gasUsed).toEqual(EXP_GAS);
    
                        const spentGasLegacy = legacyReceipt.gasUsed * legacyReceipt.gasPrice;
                        //check balances
                        const senderBalancePostLegacy = (await provider.getBalance(ALITH_ADDRESS));
                        expect(senderBalancePostLegacy).toEqual(alithBalance - BigInt(AMOUNT) - spentGasLegacy)


                        const receiverBalancePostLegacy = (await provider.getBalance(BALTATHAR_ADDRESS));
                        expect(receiverBalancePostLegacy).toEqual(baltatharBalance + BigInt(AMOUNT));
                    }
                }
            }
        });

        it({
            id: '02',
            title: 'Test of a Post-EIP-1559 Transaction',
            test: async function () {
                const provider = context.ethers().provider;
                const alithWallet = new Wallet(ALITH_PRIVATE_KEY, provider);
                // update balances
                alithBalance = await provider.getBalance(ALITH_ADDRESS);
                baltatharBalance = await provider.getBalance(BALTATHAR_ADDRESS);

                const newTx = await alithWallet.sendTransaction({value: AMOUNT, to: BALTATHAR_ADDRESS, type: 2}); //type 2 is new tx
                await newTx.wait();
                const receiptNewTx = await provider.getTransactionReceipt(newTx.hash);
                if (receiptNewTx) {
                    expect(receiptNewTx?.gasUsed).toEqual(EXP_GAS);
                    const spentGasNewTx = receiptNewTx?.gasUsed * receiptNewTx?.gasPrice;
    
                    //check balances
                    const senderBalancePostTx = (await provider?.getBalance(ALITH_ADDRESS));
                    expect(senderBalancePostTx).toEqual(alithBalance - BigInt(AMOUNT) - spentGasNewTx);
                }
            }
        })
    }
})
