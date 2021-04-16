use bytes::Bytes;
use std::collections::VecDeque;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Block {
    curr_hash: Bytes,
    data: Bytes,
    prev_hash: Bytes,
}

impl Block {
    fn new(data: Bytes, prev_hash: Bytes) -> Self {
        Self {
            curr_hash: Bytes::from(Sha256::digest(&[data.clone(), prev_hash.clone()].concat()).to_vec()),
            data,
            prev_hash,
        }
    }

    fn genesis() -> Self {
        Self::new(Bytes::from("Genesis"), Bytes::new())
    }

    fn get_hash(&self) -> Bytes {
        self.curr_hash.clone()
    }

    pub fn details(&self) -> String {
        format!(
            "\ncurr_hash: 0x{}\nblck data: {:?}\nprev_hash: 0x{}\n",
            hex::encode(self.curr_hash.to_vec()),
            self.data,
            hex::encode(self.prev_hash.to_vec())
        )
    }
}

pub struct BlockChain {
    pub blocks: VecDeque<Block>,
    curr_hash: Bytes,
}

impl BlockChain {
    pub fn new() -> Self {
        let block = Block::genesis();
        let mut blocks = VecDeque::new();
        blocks.push_back(block.clone());
        Self {
            curr_hash: block.get_hash(),
            blocks,
        }
    }

    pub fn add_block(&mut self, data: Bytes) {
        let new_block = Block::new(data, self.curr_hash.clone());
        self.curr_hash = new_block.get_hash();
        self.blocks.push_front(new_block);
    }
}
