use bytes::Bytes;
use sha2::{Digest, Sha256};

pub struct Block {
    hash: Bytes,
    data: Bytes,
    prev_hash: Bytes,
}

impl Block {
    fn new(data: Bytes, prev_hash: Bytes) -> Self {
        Self {
            hash: Bytes::from(Sha256::digest(&[data.clone(), prev_hash.clone()].concat()).to_vec()),
            data,
            prev_hash,
        }
    }

    fn genesis() -> Self {
        Self::new(Bytes::from("Genesis"), Bytes::new())
    }

    fn get_hash(&self) -> Bytes {
        self.hash.clone()
    }

    pub fn details(&self) -> String {
        format!(
            "hash: {:?}\ndata: {:?}\nprev_hash: {:?}",
            self.hash, self.data, self.prev_hash
        )
    }
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
    curr_hash: Bytes,
}

impl BlockChain {
    pub fn new() -> Self {
        let block = Block::genesis();
        Self {
            curr_hash: block.get_hash(),
            blocks: vec![block],
        }
    }

    pub fn add_block(&mut self, data: Bytes) {
        let new_block = Block::new(data, self.curr_hash.clone());
        self.curr_hash = new_block.get_hash();
        self.blocks.push(new_block);
    }
}
