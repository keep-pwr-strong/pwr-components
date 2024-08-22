import { PWRJS } from "@pwrjs/core"
import { Buffer } from "buffer"

const pwrjs = new PWRJS("https://pwrrpc.pwrlabs.io/")
  
export class Handler {
  constructor() {
    this.baseLayerStartingBlock = 129299; // TODO: change this to the correct block number
    this.vmId = 70;
  }

  async handle() {
    setInterval(async () => {
      try {
        const latestBlockNumber = await pwrjs.getLatestBlockNumber();
        if (this.baseLayerStartingBlock < latestBlockNumber) {
          await new Promise((resolve) => setTimeout(resolve, 100)); // To avoid spamming the RPC node
          const txns = await pwrjs.getVMDataTransactions(this.baseLayerStartingBlock, latestBlockNumber, this.vmId);
          if (!txns || txns.length === 0) {
            this.baseLayerStartingBlock++;
            return;
          }

          let extractedTxns = this.extractWrappedTransactions(txns);
          extractedTxns = this.filterTransactions(extractedTxns);
          this.processTransactions(extractedTxns, txns[0].timestamp);

          this.baseLayerStartingBlock++;
          // TODO: save progress to disk
        } else {
          await new Promise((resolve) => setTimeout(resolve, 100));
        }
      } catch (error) {
        console.error(error);
      }
    }, 1000);
  }

  extractWrappedTransactions(wrappedTxns) {
    const transactions = [];

    for (let txn of wrappedTxns) {
      const dataHex = txn.data;
      const data = Buffer.from(dataHex.substring(2), 'hex');

      let offset = 0;
      try {
        while (offset < data.length) {
          const txnLength = data.readInt32BE(offset);
          offset += 4;
          const txnBytes = data.slice(offset, offset + txnLength);
          transactions.push(txnBytes);
          offset += txnLength;
        }
      } catch (error) {
        console.error("Invalid transaction length");
      }
    };

    return transactions;
  }

  filterTransactions(txns) {
    // TODO: Filter transactions according to the rule of your L1
    return txns;
  }

  processTransactions(txns, timestamp) {
    // TODO: Process the transaction according to the rule of your L1
    // This might include creating a new block for these transactions
    // You can use the PWR base layer timestamp of these transactions as the timestamp of the L1 block
  }
}
