use std::borrow::Borrow;
use chrono::Utc;
use log::{error, info, warn};
use sha2::{Digest, Sha256};


pub struct BlockChain {
    blocks: Vec<Block>,
}

const DIFFICULTY_PREFIX: &str = "00";

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: Vec::from([BlockChain::genesis()])
        }
    }
    fn genesis() -> Block {
        Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 1,
            hash: String::from("0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43"),
        }
    }

    pub fn add_block(&mut self, data: String) -> &Block {
        let last = self.blocks.last().expect("always a last block");
        let block = self.mk_block(last.id.clone(), last.hash.clone(), data).expect("error creating block");
        self.blocks.push(block);
        self.blocks.last().expect("has a block")
    }
    fn mk_block(&mut self, id: u64, last_hash: String, data: String) -> Option<Block> {
        let block = Block::new(id + 1,last_hash,data);
        let latest_block = self.blocks.last().expect("there is at least one block");
        if self.is_block_valid(&block, latest_block) {
            return Some(block)
        }
        None

    }


    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        }
        let hash = &hex::decode(&block.hash).expect("can decode from hex");
        let invalid_difficulty = !hash_to_binary_representation(hash).starts_with(DIFFICULTY_PREFIX);
        if invalid_difficulty {
            warn!("block with id: {} has invalid difficult", block.id);
            return false;
        }
        if block.id != previous_block.id + 1 {
            warn!("block id: {} is not the next block after the latest: {}",block.id,previous_block.id);
            return false;
        }
        let block_hash = hex::encode(
            BlockChain::calculate_hash(block.id, block.timestamp, &block.previous_hash, &block.data, block.nonce)
        );
        if block_hash != block.hash {
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }

    fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
        let data = serde_json::json!({
            "id": id,
            "previous_hash": previous_hash,
            "data": data,
            "timestamp": timestamp,
            "nonce": nonce
        });
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }
    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 1..chain.len() {
            let first = chain.get(i - 1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !self.is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        info!("mining block...");

        for nonce in 0..u64::MAX{
            let hash = BlockChain::calculate_hash(id, timestamp, previous_hash, data, nonce);
            let binary_hash = hash_to_binary_representation(&hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                return (nonce, hex::encode(hash));
            }
        }
        panic!("unable to to mine block")
    }
}
fn hash_to_binary_representation(hash: &[u8]) -> String {
    // editor says join is a thing. Compiler says otherwise
    hash.iter().map(|c| format!("{:b}", c)).fold("".to_string(), |cur, nxt| cur + &nxt)
}

pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}


impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let (nonce,hash) = BlockChain::mine_block(id, Utc::now().timestamp(), &previous_hash, &data);
        Block{
            id,
            hash,
            previous_hash,
            timestamp: Utc::now().timestamp(),
            data,
            nonce,
        }
    }
}


