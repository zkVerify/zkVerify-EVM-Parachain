import { describeSuite, expect, beforeAll, customDevRpcRequest } from "@moonwall/cli";
import { BALTATHAR_ADDRESS } from "@moonwall/util";
import {parseEther, Wallet} from "ethers";
import { ApiPromise } from "@polkadot/api";
export const tVFY = 1_000_000_000_000_000_000n;


describeSuite({
    id: "D11",
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
                context.polkadotJs().tx.balances.transferAllowDeath(randomWeb3Account.address, tVFY)
            );
            expect(result!.successful, result!.error?.name).to.be.true;

            polkadotJs = context.polkadotJs();
        });

        it({
            id: "001",
            title: "Check that debug_traceTransaction is not implemented",
            test: async function () {
                const signer = new Wallet(privateKey, context.ethers().provider);

                const tx = await signer.sendTransaction({
                    to: BALTATHAR_ADDRESS,
                    value: parseEther("0.01"),
                    nonce: await signer.getNonce(),
                });

                await context.createBlock();

                expect(
                    async () => await customDevRpcRequest("debug_traceTransaction", [tx?.hash]),
                    "debug_traceTransaction should have failed but it worked instead"
                  ).rejects.toThrowError("Method not found");

            },
        });


        it({
            id: "002",
            title: "Check that debug_traceBlockByHash is not implemented",
            test: async function () {
                const block = (await polkadotJs.rpc.chain.getBlock()).block.header.hash;

                expect(
                    async () => await customDevRpcRequest("debug_traceBlockByHash", [block]),
                    "debug_traceBlockByHash should have failed but it worked instead"
                  ).rejects.toThrowError("Method not found");
                                

            },
        });

        it({
            id: "003",
            title: "Check that debug_traceBlockByHash is not implemented",
            test: async function () {
                
                const block_number = context.web3().utils.toHex(await context.viem().getBlockNumber());

                expect(
                    async () => await customDevRpcRequest("debug_traceBlockByNumber", [block_number]),
                    "debug_traceBlockByNumber should have failed but it worked instead"
                  ).rejects.toThrowError("Method not found");
                                

            },
        });


      },
});
