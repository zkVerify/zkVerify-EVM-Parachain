import { describeSuite, expect } from "@moonwall/cli";
import { ALITH_PRIVATE_KEY, BALTATHAR_ADDRESS } from "@moonwall/util"
import { Wallet } from "ethers";

describeSuite({
    id: 'D10',
    title: 'Test suite for invalid nonces',
    foundationMethods: 'dev',
    testCases: async ({ context, it, log }) => {
        it({
            id: '01',
            title: "Test Invalid Nonce",
            test: async () => {
                const provider = context.ethers().provider;
                const alithWallet = new Wallet(ALITH_PRIVATE_KEY, provider);
                const initialNonce = await alithWallet.getNonce();

                // send tx with higher nonce
                const badTx = await alithWallet.sendTransaction({value: 100000, to: BALTATHAR_ADDRESS, nonce: initialNonce+2});
                //send a tx with next nonce and wait it is mined
                const block1 = await context.createBlock();
                await alithWallet.sendTransaction({value: 100000, to: BALTATHAR_ADDRESS, nonce: initialNonce});
                const block2 = await context.createBlock();
            
                const nonce = await alithWallet.getNonce();
                expect(badTx.isMined()).toBeFalsy();
                // check if there is a method to see if block2 is not created
                
                //now send another tx with correct nonce
                const tx3 = await alithWallet.sendTransaction({value: 100000, to: BALTATHAR_ADDRESS, nonce: initialNonce+1});
                await context.createBlock();
                

                const finalNonce = await alithWallet.getNonce();
                expect(finalNonce).not.toEqual(initialNonce + 3);
            }
        })
    }
})
