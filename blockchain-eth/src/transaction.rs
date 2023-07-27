use pqcrypto_dilithium::dilithium2::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Debug, Clone)]
pub struct Transaction {
  pub sender: String,
  pub receiver: String,
  pub amount: i32,
}

impl Transaction {
  pub fn calculate_hash(&self) -> Vec<u8> {
    let format = format!("{:?}{:?}{:?}", self.sender, self.receiver, self.amount);
    let mut hasher = Sha256::new();
    hasher.input_str(&format);
    let mut hash = vec![];
    hash.extend(hasher.result_str().bytes());
    return hash;
  }

  pub fn sign(&mut self, secret_key: SecretKey) -> SignedMessage {
    sign(&self.calculate_hash(), &secret_key)
  }
}
