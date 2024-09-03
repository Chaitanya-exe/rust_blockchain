use std::time::SystemTime;
use log::info;
use crypto::{digest::Digest, sha2::Sha256};
use serde::{Serialize, Deserialize};
use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block{
    timestamp: u128,
    height: usize,
    transaction: String,
    prev_block_hash: String,
    hash: String,
    nonce: i32
}

const TARGET_HEXT: usize = 4;



impl Block{
    pub fn new_genesis_block() -> Block{
        Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
    }

    pub fn new_block(data: String, prev_block_hash: String, height: usize)-> Result<Block>{
        let timestamp = SystemTime::now().
            duration_since(SystemTime::UNIX_EPOCH)?.
            as_millis();

        let mut block = Block{
            timestamp,
            transaction: data,
            height,
            nonce: 0,
            prev_block_hash,
            hash: String::new(),
        };
        block.run_proof_if_work()?;
        Ok(block)
    }

    fn run_proof_if_work(&mut self) -> Result<()>{
        info!("Mining the Block");
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>>{
        let content = (
            self.prev_block_hash.clone(),
            self.transaction.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce
        );
        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }

    fn validate(&self) -> Result<bool>{
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1:Vec<u8> = vec![];
        vec1.resize(TARGET_HEXT, '0' as u8);
        Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
    }

    pub fn get_hash(&self) -> String{
        self.hash.clone()
    }
    
    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }
}

