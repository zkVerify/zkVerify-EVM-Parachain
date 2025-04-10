import { describeSuite, expect } from "@moonwall/cli";
import { ALITH_PRIVATE_KEY, ALITH_ADDRESS, BALTATHAR_ADDRESS } from "@moonwall/util"
import { Wallet } from "ethers";

/**
 * This custom script is intended to check nonce gaps in transactions
 * - send a tx with current nonce + 2 and another with the  correct  nonce
 * - check only the second is mined
 * - send another tx with nonce + 1 to fill the gap
 * - check all of them are mined
 * 
 */
describeSuite({
    id: 'D10',
    title: 'Test suite for invalid nonces. Intended to check nonce gaps in transactions',
    foundationMethods: 'dev',

    testCases: async ({ context, it, log }) => {
        it({
            id: '01',
            title: "Test Invalid Nonce",
            test: async () => {
                const provider = context.ethers().provider;
                const alithWallet = new Wallet(ALITH_PRIVATE_KEY, provider);
                const initialNonce = await context.viem().getTransactionCount({ address: ALITH_ADDRESS });
                var block_number = (await context.viem().getBlockNumber({ cacheTime: 0 }));
                log("Initial nonce: " + initialNonce + ", block number: " + block_number)
                // send tx with higher nonce
                const future_tx = await alithWallet.sendTransaction({ value: 100000, to: BALTATHAR_ADDRESS, nonce: initialNonce + 2 });
                log("future tx hash: " + future_tx.hash)
                //send a tx with next nonce and wait it is mined
                await alithWallet.sendTransaction({ value: 100000, to: BALTATHAR_ADDRESS, nonce: initialNonce });
                await context.createBlock();
                block_number = (await context.viem().getBlockNumber({ cacheTime: 0 }));
                const mediumNonce = await context.viem().getTransactionCount({ address: ALITH_ADDRESS });
                log("Medium nonce: " + mediumNonce + ", block number: " + block_number);
                //now send another tx with correct nonce so that the future one gets mined as well
                const recover_tx = await alithWallet.sendTransaction({ value: 100000, to: BALTATHAR_ADDRESS, nonce: initialNonce + 1 });
                await context.createBlock();
                // check we have the future tx mined in the block together with the recovering tx
                // and only them
                const block = await context.viem().getBlock({ blockTag: "latest" });
                log(`The block transactions: ${JSON.stringify(block.transactions, null, 2)}`);
                expect(block.transactions.length).toEqual(2);
                const tx1 = block.transactions.find(h => h === future_tx.hash);
                expect(tx1).toBeDefined();
                const tx2 = block.transactions.find(h => h === recover_tx.hash);
                expect(tx2).toBeDefined();
                const finalNonce = await context.viem().getTransactionCount({ address: ALITH_ADDRESS });
                block_number = (await context.viem().getBlockNumber({ cacheTime: 0 }));
                log("Final nonce: " + finalNonce + ", block number: " + block_number)
                expect(finalNonce).toEqual(initialNonce + 3);
            }
        });
    },
})
