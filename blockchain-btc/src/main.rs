mod blockchain;
mod block;
mod transaction;
mod wallet;

use crate::transaction::Transaction;
use crypto::{digest::Digest, sha2::Sha256};
use block::*;
use blockchain::*;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.create_genesis();

    let system = wallet::PqBtcWallet::new();
    let alice = wallet::PqBtcWallet::new();
    let bob = wallet::PqBtcWallet::new();
    let miner = wallet::PqBtcWallet::new();

    let mut first_transaction = Transaction {
      sender: system.address.to_string(),
      receiver: alice.address.clone().to_string(),
      amount: 150,
    };

    let mut second_transaction = Transaction {
      sender: alice.address.clone().to_string(),
      receiver: bob.address.clone().to_string(),
      amount: 50,
    };

    let first_sign = first_transaction.sign(system.secret_key);
    let second_sign = second_transaction.sign(alice.secret_key);

    blockchain.approve_and_send(first_transaction, first_sign, system.public_key);
    blockchain.approve_and_send(second_transaction, second_sign, alice.public_key);

    blockchain.mine_pending_transactions(miner.address.clone().to_string());

    /*  let mut third_transaction = Transaction {
      sender: bob.address.clone().to_string(),
      receiver: alice.address.clone().to_string(),
      amount: 25,
    };

    let third_sign = third_transaction.sign(bob.secret_key);
    blockchain.approve_and_send(third_transaction, third_sign, bob.public_key);

    blockchain.mine_pending_transactions(miner.address.clone().to_string());  */
    
    println!("{:?}", blockchain);
    println!("Balance of Alice: {:?}", blockchain.clone().get_balance(&alice.address));
    println!("Balance of Bob: {:?}", blockchain.clone().get_balance(&bob.address));
    println!("Balance of Miner: {:?}", blockchain.clone().get_balance(&miner.address.clone().to_string()));
}

fn calculate_hash(block: &Block) -> String {
  let s = format!("{}{}{}", block.timestamp, block.previous_hash, block.nonce);
  let mut hasher = Sha256::new();
  hasher.input_str(&s);
  return hasher.result_str();
}
