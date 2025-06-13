import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { CHARLETH_ADDRESS, BALTATHAR_ADDRESS, alith, setupLogger } from "@moonwall/util";
import {parseEther, formatEther, Wallet} from "ethers";
import { BN } from "@polkadot/util";
import { ApiPromise } from "@polkadot/api";
import {tVFY} from "../../helpers/constants";


describeSuite({
    id: "D01",
    title: "Dev test suite",
    foundationMethods: "dev",
    testCases: ({ it, context, log }) => {
        let polkadotJs: ApiPromise;
        const anotherLogger = setupLogger("anotherLogger");
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
            title: "Checking that launched node can create blocks",
            test: async function () {
                const block = (await polkadotJs.rpc.chain.getBlock()).block.header.number.toNumber();
                await context.createBlock();
                const block2 = (await polkadotJs.rpc.chain.getBlock()).block.header.number.toNumber();
                log(`Original block #${block}, new block #${block2}`);
                expect(block2).to.be.greaterThan(block);
            },
        });

        it({
            id: "002",
            title: "Checking that substrate txns possible",
            timeout: 20000,
            test: async function () {
                const balanceBefore = (await polkadotJs.query.system.account(BALTATHAR_ADDRESS)).data.free;

                await polkadotJs.tx.balances
                    .transferAllowDeath(BALTATHAR_ADDRESS, parseEther("1"))
                    .signAndSend(alith);

                await context.createBlock();
                const balanceAfter = (await polkadotJs.query.system.account(BALTATHAR_ADDRESS)).data.free;
                log(
                    `Baltathar account balance before ${formatEther(
                        balanceBefore.toBigInt()
                    )} tVFY, balance after ${formatEther(balanceAfter.toBigInt())} tVFY`
                );
                expect(balanceBefore.lt(balanceAfter)).to.be.true;
            },
        });

        it({
            id: "003",
            title: "Checking that sudo can be used",
            test: async function () {
                await context.createBlock();
                await context.createBlock();
                const tx = polkadotJs.tx.rootTesting.fillBlock(60 * 10 ** 7);
                await polkadotJs.tx.sudo.sudo(tx).signAndSend(alith);

                await context.createBlock();
                const blockFill = await polkadotJs.query.system.blockWeight();
                expect(blockFill.normal.refTime.unwrap().gt(new BN(0))).to.be.true;
            },
        });

        it({
            id: "004",
            title: "Can send Ethers txns",
            test: async function () {
                const balanceBefore = (await polkadotJs.query.system.account(BALTATHAR_ADDRESS)).data.free;
                const signer = new Wallet(privateKey, context.ethers().provider);

                await signer.sendTransaction({
                    to: BALTATHAR_ADDRESS,
                    value: parseEther("0.01"),
                    nonce: await signer.getNonce(),
                });
                await context.createBlock();
                anotherLogger("Example use of another logger");
                const balanceAfter = (await polkadotJs.query.system.account(BALTATHAR_ADDRESS)).data.free;
                log(
                    `Baltathar account balance before ${formatEther(
                        balanceBefore.toBigInt()
                    )} tVFY, balance after ${formatEther(balanceAfter.toBigInt())} tVFY`
                );
                expect(balanceBefore.lt(balanceAfter)).to.be.true;
            },
        });

        it({
            id: "005",
            title: "Testing out Create block and listen for event",
            timeout: 30000,
            test: async function () {
                const expectEvents = [
                    polkadotJs.events.system.ExtrinsicSuccess,
                    polkadotJs.events.balances.Transfer,
                    // polkadotJs.events.authorFilter.EligibleUpdated,
                ];

                await context.createBlock(
                    polkadotJs.tx.balances.transferAllowDeath(CHARLETH_ADDRESS, parseEther("2")),
                    { expectEvents, logger: log }
                );
            },
        });

        it({
            id: "006",
            title: "Testing out Create block and analyse failures",
            timeout: 30000,
            test: async function () {
                const { result } = await context.createBlock(
                    polkadotJs.tx.balances.forceTransfer(
                        BALTATHAR_ADDRESS,
                        CHARLETH_ADDRESS,
                        parseEther("3")
                    ),
                    { allowFailures: true, logger: log }
                );

                expect(
                    result.events.find((evt) => polkadotJs.events.system.ExtrinsicFailed.is(evt.event)),
                    "No Event found in block"
                ).toBeTruthy();
            },
        });
    },
});
