import { addTransactionToSubmit, initTxnSubmission } from "./wrapper.js";
import { Handler } from "./handler.js";
import crypto from "crypto";

function createDummyTransaction() {
  // Create a random byte array to simulate a transaction
  return crypto.randomBytes(32);
}

addTransactionToSubmit(createDummyTransaction());
initTxnSubmission();

let handleNode = new Handler();
handleNode.handle();
