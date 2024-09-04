use std::collections::HashMap;
use crate::block::Block;
use crate::errors::Result;
use log::info;
use crate::transaction::{TXOutput, Transaction};
use sled::Db;

const GENESIS_COINBASE: String = String::new();

const TARGET_HEXT: usize = 4;
#[derive(Debug, Clone)]
pub struct Blockchain{
    current_hash: String,
    db: Db,
}

pub struct BlockchainIter<'a>{
    current_hash: String,
    bc: &'a Blockchain
}

impl Blockchain{
    pub fn new() -> Result<Blockchain>{
        info!("Open Blockchain");

        let db = sled::open("data/blocks")?;
        let hash = db
            .get("LAST")?
            .expect("Must create a new block database first");
        info!("Found block database");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain{
            current_hash:lasthash,
            db
        })
    }

    pub fn create_blockchain(address: String) -> Result<Blockchain>{
        info!("Creating new Blockchain");

        let db = sled::open("data/blocks")?;
        info!("Create new block database");
        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE))?;
        let genesis = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain{
            current_hash: genesis.get_hash(),
            db
        };
        bc.db.flush()?;
        Ok(bc)
    }
       

    pub fn add_block(&mut self, data: Vec<Transaction>) -> Result<()>{
        let lasthash = self.db.get("LAST")?.unwrap();
        
        let block = Block::new_block(data, String::from_utf8(lasthash.to_vec())?, TARGET_HEXT);
        let new_block = block.unwrap();
        self.db.insert(new_block.get_hash(),bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }

    pub fn find_unspent_transaction(&self, address: &str) -> Vec<Transaction> {
        let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspent_TXs: Vec<Transaction> = Vec::new();

        for block in self.iter() {
            for tx in block.get_transaction(){
                for index in 0..tx.vout.len(){
                    if let Some(ids) = spent_TXOs.get(&tx.id){
                        if ids.contains(&(index as i32)){
                            continue;
                        }
                    }

                    if tx.vout[index].can_be_unlock_with(address.to_string()){
                        unspent_TXs.push(tx.to_owned());
                    }

                    if !tx.is_coinbase() {
                        for i in &tx.vin{
                            if i.can_be_unlock_with(address.to_string()){
                                match spent_TXOs.get_mut(&i.txid){
                                    Some(v) => {
                                        v.push(i.vout);
                                    }
                                    None => {
                                        spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        unspent_TXs
         
    }

    pub fn find_UTXO(&self, address: &str) -> Vec<TXOutput>{
        let mut utxos = Vec::<TXOutput>::new();
        let unspent_TXs = self.find_unspent_transaction(address);

        for tx in unspent_TXs{
            for out in &tx.vout{
                if out.can_be_unlock_with(address.to_string()){
                    utxos.push(out.clone());
                }
            }
        }
        utxos
    }

    pub fn iter(&self) -> BlockchainIter{
        BlockchainIter{
            current_hash: self.current_hash.clone(),
            bc: &self
        }
    }
}

impl<'a> Iterator for BlockchainIter<'a>{
    type Item = Block;

    fn next(&mut self)-> Option<Self::Item>{
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash){
            return match encode_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b){
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else{
                        None
                    }
                }
                None => None
            };
        }
        None
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_add_block(){
        let mut b = Blockchain::new().unwrap();

        // b.add_block("data1".to_string());
        // b.add_block("data2".to_string());
        // b.add_block("data3".to_string());

        for item in b.iter(){
            println!("item {:?}", item);
        }
    }
}