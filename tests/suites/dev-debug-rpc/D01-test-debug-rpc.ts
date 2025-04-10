import { describeSuite, expect, beforeAll, customDevRpcRequest } from "@moonwall/cli";
import { BALTATHAR_ADDRESS} from "@moonwall/util";
import {parseEther, Wallet} from "ethers";
import { ApiPromise } from "@polkadot/api";
export const ZEN = 1_000_000_000_000_000_000n;


describeSuite({
    id: "D01",
    title: "Debug RPC methods",
    foundationMethods: "dev",
    testCases: ({ it, context, log }) => {
        let polkadotJs: ApiPromise;
        let privateKey: `0x${string}`;
        let randomWeb3Account: any;

        beforeAll(async () => {
            randomWeb3Account = context.web3().eth.accounts.create();
            privateKey = randomWeb3Account.privateKey;
            const { result, block } = await context.createBlock(
                context.polkadotJs().tx.balances.transferAllowDeath(randomWeb3Account.address, ZEN)
            );
            expect(result!.successful, result!.error?.name).to.be.true;

            polkadotJs = context.polkadotJs();
        });

        it({
            id: "001",
            title: "Test debug_traceTransaction",
            test: async function () {

                // TODO This test is not completed yet because the tx is not found even if it was correctly mined
                const signer = new Wallet(privateKey, context.ethers().provider);

                const tx = await signer.sendTransaction({
                    to: BALTATHAR_ADDRESS,
                    value: parseEther("0.01"),
                    nonce: await signer.getNonce(),
                });

                await context.createBlock();
                const block = await context.viem().getBlock({ blockTag: "latest" });

                log(`block transactions #${JSON.stringify(block.transactions)}`);
                log(`tx.hash #${JSON.stringify(tx.hash)}`);

                const trace = await customDevRpcRequest("debug_traceTransaction", [tx.hash]);
                log(`trace #${JSON.stringify(trace)}`);



            },
        });



      },
});
