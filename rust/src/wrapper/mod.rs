use pwr_rs::{Wallet, RPC};
use pwr_rs::transaction::NewTransactionData;
use std::sync::{Arc, Mutex};

pub struct Wrapper {
  pub rpc_node: RPC,
  pub wrapper_wallet: Wallet,
  pub vm_id: u64,
  pub transactions_to_submit: Arc<Mutex<Vec<Vec<[u8; 32]>>>>,
}

impl Wrapper {
  pub async fn new() -> Self {
    let rpc_node = RPC::new("https://pwrrpc.pwrlabs.io/").await.unwrap();
    let wrapper_wallet = Wallet::from_hex("0x9D4428C6E0638331B4866B70C831F8BA51C11B031F4B55EED4087BBB8EF0151F")
      .unwrap();
    Self {
      rpc_node,
      wrapper_wallet,
      vm_id: 45,
      transactions_to_submit: Arc::new(Mutex::new(Vec::new())),
    }
  }

  pub fn add_transaction_to_submit(&self, txn: Vec<[u8; 32]>) {
    let mut transactions = self.transactions_to_submit.lock().unwrap();
    if !transactions.contains(&txn) {
      transactions.push(txn);
    }
  }

  pub fn has_transaction(&self, txn: &Vec<[u8; 32]>) -> bool {
    let transactions = self.transactions_to_submit.lock().unwrap();
    transactions.contains(&txn.to_vec())
  }

  pub async fn init_txn_submission(&self) {
    let transactions_to_submit = Arc::clone(&self.transactions_to_submit);
    let wallet = self.wrapper_wallet.clone();
    let vm_id = self.vm_id;

    let txns: Vec<_> = {
      let transactions = transactions_to_submit.lock().unwrap();
      transactions.clone()
    };

    if txns.is_empty() {
      panic!("Wrong");
    }

    let mut _pwr_txn_size = txns.len() * 4;
    for txn in &txns {
      _pwr_txn_size += txn.len();
    }

    let mut buffer = Vec::with_capacity(_pwr_txn_size);
    for txn in &txns {
      for slice in txn {
        buffer.extend_from_slice(slice);
      }
    }

    let new_tx = NewTransactionData::VmData { 
      vm_id: vm_id,
      data: buffer.clone()
    };
    let tx_hash = self.rpc_node.broadcast_transaction(&new_tx, &wallet).await.unwrap();
    println!("Transaction Hash: {tx_hash}");
  }
}
