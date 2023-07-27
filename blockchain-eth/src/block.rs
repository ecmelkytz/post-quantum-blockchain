use crate::transaction::Transaction;
use crate::calculate_hash;
use chrono::prelude::*;

const DIFFICULTY: usize= 2;

#[derive(Debug, Clone)]
pub struct Block {
  pub previous_hash: String,
  pub timestamp: i64,
  pub transactions: Vec<Transaction>,
  pub hash: String,
  pub nonce: u64,
}

impl Block {
  #[allow(dead_code)]
  pub fn new(transactions: Vec<Transaction>) -> Self {
    Self {
      timestamp: Utc::now().timestamp(),
      previous_hash: String::new(),
      hash: String::new(),
      transactions: transactions,
      nonce: 0u64,
    }
  }

  pub fn mining(&mut self) {
    let prefix = "0".repeat(DIFFICULTY);
    while self.hash.get(0..DIFFICULTY).unwrap() != prefix {
      self.nonce += 1;
      self.hash = calculate_hash(self);
    }
  }
}
