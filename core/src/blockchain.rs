use util::sleder::Sleder;

use crate::block::Block;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct BlockChain {
    pub genesis_hash: [u8; 32],
    pub curr_hash: [u8; 32],
    pub curr_height: u64,
}

impl BlockChain {
    /// 创建新链
    pub fn new() -> Self {
        let genesis_block = crate::block::Block::new_genesis_block();

        let blockchain = BlockChain {
            genesis_hash: genesis_block.hash,
            curr_hash: genesis_block.hash,
            curr_height: 1,
        };
        // 持久化链
        BlockChain::save_blockchain(&blockchain);
        blockchain
    }

    /// 写入新区块
    fn write_block(block: Block) {
        // 校验
        // 持久化
    }

    fn save_blockchain(blockchain: &BlockChain) {
        if let Some(sled_db) = Sleder::open() {
            sled_db
                .insert(blockchain.genesis_hash, util::coder::serialize(blockchain))
                .unwrap();
        }
    }
}

#[test]
fn test() {
    // println!("{:?}", BlockChain::new());
    Sleder::show_all();
}
