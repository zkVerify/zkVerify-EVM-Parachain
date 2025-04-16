import { describeSuite, expect} from "@moonwall/cli";
import { ALITH_ADDRESS, BALTATHAR_ADDRESS, alith } from "@moonwall/util";
// import { u8aToString } from "@polkadot/util"

describeSuite({
    id: "D08",
    title: "Substrate custom asset",
    foundationMethods: "dev",
    testCases: ({ context, it, log }) => {
        const assetId = 1;

        it({
            id: "T01",
            title: "Test metadata",
            test: async () => {
                const { name, symbol, decimals, isFrozen } = await context.polkadotJs().query.assets.metadata(assetId);
                // expect(u8aToString(name)).toBe("ZK Verify Wrapper");
                // expect(symbol.toUtf8()).toBe("xcZKV");
                expect(decimals).toBe(18);
                expect(isFrozen).toBe(true);
            }
        })

        it({
            id: "T02",
            title: "Test of a simple transfer of a custom asset",
            test: async () => {
                // RangeError: Invalid string length
                const AMOUNT_TO_TRANSFER = 500;
                const alithBalance = await context.polkadotJs().query.assets.account(assetId, ALITH_ADDRESS);
                expect(alithBalance.unwrap().balance).toBe(1000);

                const baltatharBalanceBefore = await context.polkadotJs().query.assets.account(assetId, BALTATHAR_ADDRESS);
                expect(baltatharBalanceBefore.unwrap().balance).toBe(0);


                const transfer = await context.polkadotJs().tx.assets.transfer(assetId, BALTATHAR_ADDRESS, AMOUNT_TO_TRANSFER);
                await transfer.signAndSend(alith, ({ events = [], status }) => {
                    log(`Status ${status}`)
                });

                const alithBalanceAfter = await context.polkadotJs().query.assets.account(assetId, ALITH_ADDRESS);
                expect(alithBalanceAfter.unwrap().balance).toBe(500);

                const baltatharBalanceAfter = await context.polkadotJs().query.assets.account(assetId, BALTATHAR_ADDRESS);
                expect(baltatharBalanceAfter.unwrap().balance).toBe(AMOUNT_TO_TRANSFER);

            }
        })

    },
})
