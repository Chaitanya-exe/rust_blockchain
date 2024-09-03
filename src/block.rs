use std::time::SystemTime;
use log::info;
use crypto::{digest::Digest, sha2::Sha256};

pub type Result<T> = std::result::Result<T, failure::Error>;
#[derive(Debug, Clone)]
pub struct Block{
    timestamp: u128,
    height: usize,
    transaction: String,
    prev_block_hash: String,
    hash: String,
    nonce: i32
}

const TARGET_HEXT: usize = 4;

#[derive(Debug)]
pub struct Blockchain{
    blocks: Vec<Block>
}

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
}

impl Blockchain{
    pub fn new() -> Blockchain{
        Blockchain{
            blocks: vec![Block::new_genesis_block()]
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()>{
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), TARGET_HEXT)?;
        self.blocks.push(new_block);
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_blockchain(){
        let mut b = Blockchain::new();

        b.add_block("data".to_string());
        b.add_block("data1".to_string());
        b.add_block("data23".to_string());
        dbg!(b);
    }

}