use pqcrypto_dilithium::dilithium2::*;
use crate::transaction::Transaction;
use crate::block::Block;
use crate::calculate_hash;

#[derive(Debug, Clone)]
pub struct Blockchain {
  pub blocks: Vec<Block>,
  pub pending_transactions: Vec<Transaction>,
  pub reward: i32,
}

impl Blockchain {
  pub fn new() -> Self {
    Self {
      blocks: vec![],
      pending_transactions: vec![],
      reward: 100
    }
  }

  pub fn create_genesis(&mut self) {
    let genesis_block = Block {
      timestamp: 1644525341,
      transactions: vec![],
      previous_hash: "genesis".to_string(),
      hash: "c698ee013d80c4c811bfe115969fe755a9d1530c67c699208a808b310dbec0ea".to_string(),
      nonce: 0,
    };
    self.blocks.push(genesis_block);
  }

  pub fn add_block(&mut self, mut block: Block) {
    block.previous_hash = self.blocks.last().unwrap().hash.to_string();
    block.hash = calculate_hash(&block);
    block.mining();
    self.blocks.push(block);
  }

  pub fn add_transaction(&mut self, transaction: Transaction) {
    self.pending_transactions.push(transaction);
  }

  pub fn approve_and_send(&mut self, transaction: Transaction, sign: SignedMessage, public_key: PublicKey ) {
    let verifiedmsg = open(&sign, &public_key).unwrap();
    if verifiedmsg == transaction.calculate_hash() {
      self.add_transaction(transaction);
    } else {
      panic!("You can't add transaction!");
    }
  }

  pub fn mine_pending_transactions(&mut self, miner: String) {
    self.pending_transactions.push(Transaction {
      sender: String::new(),
      receiver: miner,
      amount: self.reward,
    });

    let transactions = &self.pending_transactions;
    let block = Block::new(transactions.to_vec());
    self.add_block(block);
    self.pending_transactions = Vec::new();
  }

  pub fn get_balance(self, address: &String) -> i32 {
    let mut balance = 0i32;
    let blocks = self.blocks;
    for block in blocks.iter() {
      for trans in &block.transactions {
      if trans.sender == address.to_string() {
        balance -= trans.amount;
      }
      if trans.receiver == address.to_string() {
        balance += trans.amount;
      }
    }
  }
  return balance;
  }
}
