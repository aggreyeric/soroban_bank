
import { Contract, SorobanRpc, TransactionBuilder, Networks,
    BASE_FEE, 
    Keypair, nativeToScVal, Address } from "npm:@stellar/stellar-sdk";




let rpcUrl = "https://soroban-testnet.stellar.org"

let contractAddress = 'CC7FPIPJNE7NMZGL5IWLSYBVNMd25SDR6QR23GN2GR5EQYKVHGJSIQZ'


let params = {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET
}




const accountToScVal = (account) => new Address(account).toScVal();

const stringToSymbol = (value) => {
    return nativeToScVal(value, {type: "symbol"})
}

const numberToI128 = (value) => {
    return nativeToScVal(value, {type: "i128"})
}



const numberToU64= (value) => {
    return nativeToScVal(value, {type: "u64"})
}





export async function contractInt(functName, values) {
    const kp = Keypair.fromSecret("SCD5BHD4NPBH24J2G6EVIBNBSMWHOSGX7Jddw3K6QXCZYDWUEMQJOXXYDOFB");
    const caller = kp.publicKey();
    const provider = new SorobanRpc.Server(rpcUrl, { allowHttp: true });
    const sourceAccount = await provider.getAccount(caller);
    const contract = new Contract(contractAddress);
    let buildTx = new TransactionBuilder(sourceAccount, params)
        .addOperation(contract.call(functName, ...values))
        .setTimeout(30)
        .build();
    let prepareTx = await provider.prepareTransaction(buildTx);
    prepareTx.sign(kp);
    try {
        let sendTx = await provider.sendTransaction(prepareTx).catch(function (err) {
            return err;
        });
        if (sendTx.errorResult) {
            throw new Error("Unable to submit transaction");
        }
        if (sendTx.status === "PENDING") {
            let txResponse = await provider.getTransaction(sendTx.hash);
            while (txResponse.status === "NOT_FOUND") {
                txResponse = await provider.getTransaction(sendTx.hash);
                await new Promise((resolve) => setTimeout(resolve, 100));
            }
            if (txResponse.status === "SUCCESS") {
                let result = txResponse.returnValue;
                return result;
            }
        }
    } catch (err) {
        return err;
    }
}






export async function initialize() {
    let admin = accountToScVal("GA7KOUZLPX7T45WLNNYY6KFZZGJYMNI4H5J5TN3HHSVGL5BFQ4YCTQ4QR")
    let values = [admin]
    let result = await contractInt('init', values);
    console.log(result)

}


export async function create_account() {
    let person = accountToScVal("GA7KOUZLPX7T45WLNYNY6KFZZGJYMNI4H5J5TN3HHSVGL5BFQ4YCTQ4QR")
    let amount  = numberToI128(10)
    let values = [person, amount]
    let result = await contractInt('create_account', values);
    console.log(result)

}


export async function deposit() {
    let person = accountToScVal("GA7KOUZLPX7T45WLNYY6KFZZGJYMNI4H5J5TN3HHSVGL5BFQ4YCTQ4QR")
    let amount  = numberToI128(10)
    let values = [person, amount]
    let result = await contractInt('deposit', values);
    console.log(result)

}



export async function balance() {
    let person = accountToScVal("GA7KOUZLPX7T45WLNYY6KFZZGJYMNI4H5J5TN3HHSVGL5BFQ4YCTQ4QR")
    let values = [person]
    let result = await contractInt('myaccount_balance', values);
    console.log(result)

}




