import { Wrapper } from "./wrapper.js";
import { Handler } from "./handler.js";
import crypto from "crypto";

function createDummyTransaction() {
  // Create a random byte array to simulate a transaction
  return crypto.randomBytes(32);
}

let wrapper = new Wrapper();
wrapper.addTransactionToSubmit(createDummyTransaction());
wrapper.initTxnSubmission();

let handler = new Handler();
handler.handle();
