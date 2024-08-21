import { PWRJS, PWRWallet } from "@pwrjs/core"

const transactionsToSubmit = [];
const wrapperWallet = new PWRWallet("0x9D4428C6E0638331B4866B70C831F8BA51C11B031F4B55EED4087BBB8EF0151F");
const vmId = 50;
  
function hasTransaction(txn) {
  return transactionsToSubmit.some(t => Buffer.compare(t, txn) === 0);
}

export function addTransactionToSubmit(txn) {
  if (!hasTransaction(txn)) transactionsToSubmit.push(txn);
}

export async function initTxnSubmission() {
  let nonce = await wrapperWallet.getNonce()

  setInterval(() => {
    if (transactionsToSubmit.length === 0) return;

    const txns = [...transactionsToSubmit];
    let pwrTxnSize = txns.length * 4;

    for (const txn of txns) {
      pwrTxnSize += txn.length;
    }

    const buffer = Buffer.alloc(pwrTxnSize);
    let offset = 0;

    for (const txn of txns) {
      buffer.writeInt32BE(txn.length, offset);
      offset += 4;
      txn.copy(buffer, offset);
      offset += txn.length;
    }

    console.log(`Wrapper: Submitting ${txns.length} transactions`);

    wrapperWallet.sendVMDataTxn(vmId, buffer, nonce)
      .then(response => {
        if (!response.success) {
          console.log(`Wrapper: Error submitting txn: ${response.message}`);
        } else {
          transactionsToSubmit.splice(0, txns.length);
          console.log(`Transaction Hash: ${response.transactionHash}`);
        }
      })
      .catch(err => {
        console.log(`Wrapper: Error submitting txn: ${err.message}`);
        console.error(err);
      });
  }, 1000)
}
