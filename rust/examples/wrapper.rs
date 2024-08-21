use pwr_rs::Wallet;
use pwr_rs::rpc::RPC;
use pwr_rs::block::NewTransactionData;
use std::sync::{Arc, Mutex};

struct Wrapper {
  rpc_node: RPC,
  wrapper_wallet: Wallet,
  vm_id: u64,
  transactions_to_submit: Arc<Mutex<Vec<String>>>,
}

impl Wrapper {
  async fn new() -> Self {
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

  fn add_transaction_to_submit(&self, txns: Vec<String>) {
    let mut transactions = self.transactions_to_submit.lock().unwrap();
    for txn in txns {
      if !transactions.contains(&txn) {
        transactions.push(txn);
      }
    }
  }

  fn has_transaction(&self, txn: Vec<String>) -> bool {
    let transactions = self.transactions_to_submit.lock().unwrap();
    println!("{}", txn.iter().all(|txn| transactions.contains(txn)));
    txn.iter().all(|txn| transactions.contains(txn))
  }

  async fn init_txn_submission(&self) {
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

    let mut buffer = Vec::new();
    for txn in &txns {
      let bytes = txn.as_bytes();
      buffer.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
      buffer.extend_from_slice(bytes);
    }

    let new_tx = NewTransactionData::VmData { 
      vm_id: vm_id,
      data: buffer.clone()
    };
    let tx_hash = self.rpc_node.broadcast_transaction(&new_tx, &wallet).await.unwrap();
    println!("Transaction Hash: {tx_hash}");
  }
}

#[tokio::main]
async fn main() {
  let wrapper = Wrapper::new().await;

  let txns: Vec<String> = vec![
    String::from("alice -> bob"),
    String::from("bob -> dave"),
    String::from("carol -> alice"),
    String::from("dave -> bob"),
  ];

  wrapper.add_transaction_to_submit(txns.clone());
  wrapper.has_transaction(txns.clone());

  wrapper.init_txn_submission().await;
}
