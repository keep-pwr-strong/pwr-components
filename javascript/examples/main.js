import { addTransactionToSubmit, initTxnSubmission } from "./wrapper.js"
import crypto from "crypto"

function createDummyTransaction() {
  // Create a random byte array to simulate a transaction
  return crypto.randomBytes(32);
}

addTransactionToSubmit(createDummyTransaction());
addTransactionToSubmit(createDummyTransaction());
addTransactionToSubmit(createDummyTransaction());
addTransactionToSubmit(createDummyTransaction());

initTxnSubmission();