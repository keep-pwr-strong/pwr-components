// src/wrapper
use rust::Wrapper;
use rand::Rng;

fn create_dummy_transaction() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut transaction = [0u8; 32];
    rng.fill(&mut transaction);
    transaction
}

#[tokio::main]
async fn main() {
  let wrapper = Wrapper::new().await;

  let txns: Vec<[u8; 32]> = vec![
    create_dummy_transaction(),
    create_dummy_transaction(),
    create_dummy_transaction(),
  ];

  wrapper.add_transaction_to_submit(txns.clone());
  wrapper.has_transaction(&txns);
  wrapper.init_txn_submission().await;
}
