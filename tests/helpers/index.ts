import { cryptoWaitReady, mnemonicGenerate, mnemonicToMiniSecret, secp256k1PairFromSeed  } from '@polkadot/util-crypto';
import { u8aToHex } from '@polkadot/util';

/**
 * Return the fee paid for an Ethereum transaction,
 * given a transaction hash
 * 
 * @param provider 
 * @param txHash 
 * @returns {BigInt} fee
 */
export async function getFeeFromEthReceipt(provider, txHash, log):Promise<bigint> {
    let receipt;
    try {
        receipt = (await provider.send("eth_getTransactionReceipt" , [txHash]));

        // Check if the receipt is null, which means the transaction is not found
        if (receipt === null) {
            log(`Transaction with hash ${txHash} not found.`);
            return BigInt(0);
        }

    } catch (error) {
        log("Error fetching transaction receipt:", error);
        return BigInt(0);
    }
    const gasUsed = BigInt(receipt.gasUsed);
    const gasPrice = BigInt(receipt.effectiveGasPrice);
    const fee = BigInt(gasUsed)*BigInt(gasPrice);
    log("gasUsed="+gasUsed+", gasPrice="+gasPrice+", fee="+fee);
    return fee;
}

/**
 * Function to convert a byte array to an ASCII string
 * 
 * @param byteArray 
 * @returns {string}
 */
export function bytesToAscii(byteArray) {
    return String.fromCharCode(...byteArray);
}

/**
 * Function to convert a hex string to a byte array
 * 
 * @param hex 
 * @returns {Array<number>} byte array
 */
export function hexToBytes(hex) {
    const bytes:Array<number> = [];
    for (let i = 0; i < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16));
    }
    return bytes;
}

/**
 * Generate a random ethereum keypair
 */
export async function generateEthereumKeypair() {
    await cryptoWaitReady();
  
    //random mnemonic
    const mnemonic = mnemonicGenerate();
  
    //from mnemonic to seed
    const seed = mnemonicToMiniSecret(mnemonic);
  
    const keypair = secp256k1PairFromSeed(seed);
    const privateKey = u8aToHex(keypair.secretKey);
    const publicKey = u8aToHex(keypair.publicKey);
  
    return {
      mnemonic,
      privateKey,
      publicKey,
    };
  }

export const waitForConfirmations = function(provider, confirmations) {
    return new Promise(async (resolve, reject) =>  {
      console.log("Waiting for  "+confirmations+" confirmations...");
      var startingBlock = await provider.getBlock();
      var startingBlockNumber = startingBlock.number;   
      var startingBlockHash = startingBlock.hash;
      console.log("starting block: "+startingBlockHash+" [H: "+startingBlockNumber+"]");
      const intervalId = setInterval(async () => {
        var blockNew = await provider.getBlockNumber();  
        var delta = blockNew - startingBlockNumber;
        console.log("blocks seen: "+delta);
        if (delta>=confirmations) {      
          clearInterval(intervalId);   
          //check starting block has not been reverted
          if (await provider.getBlock(startingBlockHash) == null){
            console.log("starting block has been reverted!");
            resolve(false);
          }else{
            console.log("block "+startingBlockHash+" has "+confirmations+" confirmations");
            resolve(true);
          }   
        } 
      }, 3000); // repeat every 3 seconds
  
    });
  };
