import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { Wallet } from "ethers";
import { BALTATHAR_PRIVATE_KEY } from "@moonwall/util"

function checkEmptyObject(obj: any) : boolean {
    return JSON.stringify(obj) == "{}";
}

function toHexString(n: number) : string {
    return "0x"+n.toString(16);
}

describeSuite({
    id: "D08",
    title: "Mempool - Test empty response RPC methods",
    foundationMethods: "dev",
    testCases: ({ context, it, log }) => {

        let provider;
        let myAddress: string;

        beforeAll( () => {
            provider = context.ethers().provider;
            myAddress = context.ethers().address.toLowerCase();
        });

        it({
            id: "T01",
            title: "txpool should be empty with no tx",
            test: async () => {
                
                //txpool_status
                let rpcResponse = await provider.send("txpool_status", []);
                let pending = rpcResponse.pending;
                let queued = rpcResponse.queued;

                expect(
                    pending == "0x0",
                    `pending RPC response from txpool_status is not 0x0 as expected: ${JSON.stringify(pending)}`
                ).to.be.true;
                expect(
                    queued == "0x0",
                    `queued RPC response from txpool_status is not 0x0 as expected: ${JSON.stringify(queued)}`
                ).to.be.true;

                //txpool_content
                rpcResponse = await provider.send("txpool_content", []);
                pending = rpcResponse.pending;
                queued = rpcResponse.queued;

                expect(
                    checkEmptyObject(pending),
                    `pending RPC response from txpool_content is not empty as expected: ${JSON.stringify(pending)}`
                ).to.be.true;
                expect(
                    checkEmptyObject(queued),
                    `queued RPC response from txpool_content is not empty as expected: ${JSON.stringify(queued)}`
                ).to.be.true;
                
                //txpool_inspect
                rpcResponse = await provider.send("txpool_inspect", []);
                pending = rpcResponse.pending;
                queued = rpcResponse.queued;

                expect(
                    checkEmptyObject(pending),
                    `pending RPC response from txpool_inspect is not empty as expected: ${JSON.stringify(pending)}`
                ).to.be.true;
                expect(
                    checkEmptyObject(queued),
                    `queued RPC response from txpool_inspect is not empty as expected: ${JSON.stringify(queued)}`
                ).to.be.true;
            },
        });
        it({
            id: "T02",
            title: "txpool should be correct with an exec and a pending tx",
            test: async () => {
                const nonce = await context.ethers().getNonce();
                const hexNonce = toHexString(nonce);
                const testPendingReceiver = "0x0000000000000000000000000000000000051095";
                const testPendingValue = 1;

                const queuedNonce = nonce + 2;
                const hexQueuedNonce = toHexString(nonce+2);
                const testQueuedReceiver = "0x1111111111111111111111111111111111051095";
                const testQueuedValue = 2;

				//send pending tx without creating a block
                await context.ethers().sendTransaction({
                    to: testPendingReceiver,
                    value: testPendingValue,
                    nonce: nonce,
                    gasLimit: 21000
                })
                //send queuing tx with higher nonce
                await context.ethers().sendTransaction({
                    to: testQueuedReceiver,
                    value: testQueuedValue,
                    nonce: queuedNonce,
                    gasLimit: 21000
                })
                
                //txpool_status
                let rpcResponse = await provider.send("txpool_status", []);
                
                let pending = rpcResponse.pending;
                let queued = rpcResponse.queued;

                expect(
                    pending == "0x1",
                    `pending RPC response from txpool_status is not 0x1 as expected: ${JSON.stringify(pending)}`
                ).to.be.true;
                expect(
                    queued == "0x1",
                    `queued RPC response from txpool_status is not 0x1 as expected: ${JSON.stringify(queued)}`
                ).to.be.true;

                //txpool_content
                rpcResponse = await provider.send("txpool_content", []);

                pending = rpcResponse.pending;
                queued = rpcResponse.queued;
                expect(
                    Object.keys(pending).length == 1 &&
                    pending[myAddress] != undefined && 
                    pending[myAddress][hexNonce] != undefined &&
                    pending[myAddress][hexNonce]["to"] == testPendingReceiver &&
                    pending[myAddress][hexNonce]["value"] == testPendingValue,
                    `pending RPC response from txpool_content is not expected: ${JSON.stringify(pending)}`
                ).to.be.true;
                expect(
                    Object.keys(queued).length == 1 &&
                    queued[myAddress] != undefined && 
                    queued[myAddress][hexQueuedNonce] != undefined &&
                    queued[myAddress][hexQueuedNonce]["to"] == testQueuedReceiver &&
                    queued[myAddress][hexQueuedNonce]["value"] == testQueuedValue,
                    `queued RPC response from txpool_content is not expected: ${JSON.stringify(queued)}`
                ).to.be.true;
                
                //txpool_inspect
                rpcResponse = await provider.send("txpool_inspect", []);
                pending = rpcResponse.pending;
                queued = rpcResponse.queued;

                expect(
                    Object.keys(pending).length == 1 &&
                    pending[myAddress] != undefined && 
                    pending[myAddress][hexNonce] != undefined &&
                    pending[myAddress][hexNonce].includes(testPendingReceiver),
                    `pending RPC response from txpool_inspect is not expected: ${JSON.stringify(pending)}`
                ).to.be.true;
                expect(
                    Object.keys(queued).length == 1 &&
                    queued[myAddress] != undefined && 
                    queued[myAddress][hexQueuedNonce] != undefined &&
                    queued[myAddress][hexQueuedNonce].includes(testQueuedReceiver),
                    `queued RPC response from txpool_inspect is not expected: ${JSON.stringify(queued)}`
                ).to.be.true;
            },
        });
        it({
            id: "T03",
            title: "correct replacement only when gasPrice is higher",
            test: async () => {

                //use Baltathar (Alith nonce could be pending from the previous test)
                const bWallet = new Wallet(BALTATHAR_PRIVATE_KEY, provider);
                const bAddress = bWallet.address.toLowerCase();

                const nonce = await bWallet.getNonce();
                const feeData = await provider.getFeeData();
                const lowMaxFeePerGas = feeData.maxFeePerGas!;
                const maxFeePerGas = lowMaxFeePerGas + BigInt(100);
                const highMaxFeePerGas = lowMaxFeePerGas * BigInt(2);
                const highGasPrice = feeData.gasPrice! * BigInt(2);

                const testReceiver = "0x0000000000000000000000000000000000051095";
                const testValue = 1;
                const testReplacementReceiver = "0x1111111111111111111111111111111111051095";
                const testReplacementValue = 2;

                //send queued tx
                await bWallet.sendTransaction({
                    to: testReceiver,
                    value: testValue,
                    nonce,
                    gasLimit: 21000,
                    maxFeePerGas
                })

                log("First one good")

                // try to replace with lower gasPrice and expect error
                try {
                    const replacementTx = await bWallet.sendTransaction({
                        to: testReplacementReceiver,
                        value: testReplacementValue,
                        nonce,
                        gasLimit: 21000,
                        maxFeePerGas: lowMaxFeePerGas
                    })
                    log("Second one")
                    expect(false, `replacement tx with lower gas price was expected to fail: ${JSON.stringify(replacementTx)}`).to.be.true;
                } catch(error) {
                    expect(error.toString().includes("replacement fee too low"), `unexpected error: ${JSON.stringify(error)}`).to.be.true;

                }

                // try to replace with same gasPrice and expect error
                try {
                    const replacementTx = await bWallet.sendTransaction({
                        to: testReplacementReceiver,
                        value: testReplacementValue,
                        nonce,
                        maxFeePerGas
                    })
                    expect(false, `replacement tx with same gas price was expected to fail: ${JSON.stringify(replacementTx)}`).to.be.true;
                } catch(error) {
                    expect(error.toString().includes("replacement fee too low"), `unexpected error: ${JSON.stringify(error)}`).to.be.true;
                }

                // replace with higher gasPrice
                await bWallet.sendTransaction({
                    type: 0,
                    to: testReplacementReceiver,
                    value: testReplacementValue,
                    nonce,
                    gasLimit: 21000,
                    gasPrice: highGasPrice,
                    maxFeePerBlobGas: highMaxFeePerGas
                })

                //check successfull replacement    
                const rpcResponse = await provider.send("txpool_content", []);
                const pending = rpcResponse.pending;
                const hexNonce = toHexString(nonce);
                expect(
                    pending[bAddress] != undefined &&
                    pending[bAddress][hexNonce] != undefined &&
                    pending[bAddress][hexNonce]["to"] == testReplacementReceiver &&
                    pending[bAddress][hexNonce]["value"] == testReplacementValue,
                    `pending RPC response from txpool_content is not expected after replacement: ${JSON.stringify(pending)}`
                ).to.be.true;
            },
        });
    },
});
