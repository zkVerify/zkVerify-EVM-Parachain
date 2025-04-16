import {SessionKeys1} from "@polkadot/types/interfaces";
import {Bytes} from "@polkadot/types-codec";
import {KeyringPair} from "@polkadot/keyring/types";
import {DevModeContext, ZombieContext} from "@moonwall/cli";
import "zkv-para-evm-api-augment";
import {Debugger, log} from "debug";


export async function addSessionKey(
    account: KeyringPair,
    context: ZombieContext,
): Promise<Bytes> {

    const parachainApi = context.polkadotJs("parachain");
    const newKey = await parachainApi.rpc.author.rotateKeys();

    log('##### New key: ' + newKey);
    const signature = account.sign(newKey).toString()
    log('##### Signature: ' + signature);

    await parachainApi.tx.session.setKeys(
        newKey,
        signature,
    ).signAndSend(account);

    return newKey;
}

/**
 * Jump to the start of the next round/session.
 * Used in tests to avoid misalignment when running a single/multiple tests.
 * Needed because last block of the round/session do not accept transactions.
 * @param context DevModeContext
 */
export async function jumpToRoundStart(
    context: DevModeContext,
) {
    const parachainApi = context.polkadotJs();
    const roundInfo = (await parachainApi.query.parachainStaking.round());
    const currentHeight = (await parachainApi.rpc.chain.getHeader()).number.toNumber();

    for (let i = (currentHeight - roundInfo.first.toNumber()); i < roundInfo.length.toNumber(); i++) {
        await context.createBlock();
    }
}
