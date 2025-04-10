import {describeSuite, expect, beforeAll} from "@moonwall/cli";
import {ApiPromise, Keyring} from "@polkadot/api";

describeSuite({
    id: "Z01",
    title: "Zombie Test Suite",
    foundationMethods: "zombie",
    testCases: ({it, context, log}) => {
        let parachainApi: ApiPromise;
        let relaychainApi: ApiPromise;

        beforeAll(async () => {
            parachainApi = context.polkadotJs("parachain");
            relaychainApi = context.polkadotJs("relaychain");
        });

        it({
            id: "001",
            title: "Test zombienet is up and running",
            test: async function () {
                const localpeerid = await parachainApi.rpc.system.localPeerId(); // Returns the base-58 encoded peer id
                log('##### Local peer id: ' + localpeerid);
            },
        });

        it({
            id: "002",
            title: "Test of a simple relaychain transaction",
            test: async function () {
                // This is not really used by this test, but shows how to get the current block number
                const current_block_number = await relaychainApi.query.system.number();
                log('Current block number: ' + current_block_number);

                // Build a keyring and import Alice's credential
                // There's no documentation on that, it has been deducted from:
                // javascript/packages/orchestrator/src/test-runner/assertion.ts
                // By the way, zombie contains also the following objects / functions:
                //    ApiPromise, Keyring, WsProvider, util: utilCrypto, connect(), registerParachain()
                const keyring = new Keyring({type: 'sr25519'});
                const alice = keyring.addFromUri('//Alice');

                // Define Alice and Bob's addresses
                const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
                const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

                // Collect Alice's and Bob's free balances
                let balance_alice = (await relaychainApi.query.system.account(ALICE)).data.free;
                const balance_bob = (await relaychainApi.query.system.account(BOB)).data.free;
                log('Alice\'s balance: ' + balance_alice.toHuman());
                log('Bob\'s balance:   ' + balance_bob.toHuman());

                const TRANSFERRED_AMOUNT = 1;

                // Create an extrinsic, transferring 1 token unit to Bob.
                // We sign and submit the extrinsic, waiting for inclusion in a block.
                await relaychainApi.tx.balances
                    .transferAllowDeath(BOB, TRANSFERRED_AMOUNT)
                    .signAndSend(alice);

                await context.waitBlock(1, 'relaychain');

                // Get the updated balances
                balance_alice = (await relaychainApi.query.system.account(ALICE)).data.free;
                const balance_bob_after = (await relaychainApi.query.system.account(BOB)).data.free;
                log('Alice\'s balance after tx: ' + balance_alice.toHuman());
                log('Bob\'s balance after tx:   ' + balance_bob_after.toHuman());

                expect(balance_bob_after.toBigInt()).to.be.eq(balance_bob.toBigInt() + BigInt(TRANSFERRED_AMOUNT));
                log("Transfer successful");
            },
        });
    },
});
