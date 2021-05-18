use util::sleder::Sleder;

use crate::{
    block::{Block, BlockHeader},
    tile::Tile,
};
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
        let genesis_block: Block<[[[Tile; 3]; 3]; 3]> = BlockChain::new_genesis_block();

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
    pub fn write_block<T>(block: Block<T>) {
        // 校验
        // 持久化
    }

    /// 创建创世区块
    fn new_genesis_block() -> Block<[[[Tile; 3]; 3]; 3]> {
        // body
        let body = [[[Tile {
            point: (0, 0, 0),
            tile_type: 0,
        }; 3]; 3]; 3];
        let height = 1u64;

        let genesis_block = Block::new(height, body);
        // 持久化创世块
        BlockChain::save_block(&genesis_block);
        genesis_block
    }

    fn save_block<T: Serialize>(block: &Block<T>) {
        if let Some(sled_db) = Sleder::open() {
            sled_db
                .insert(
                    format!("block-{:?}", block.hash),
                    util::coder::serialize(block),
                )
                .unwrap();
        }
    }

    fn save_tail_block<T: Serialize>(block: &Block<T>) {
        if let Some(sled_db) = Sleder::open() {
            sled_db
                .insert("block-tail", util::coder::serialize(block))
                .unwrap();
        }
    }

    // fn get_tail_block<T: Serialize + Deserialize<'static>>() -> Option<Block<T>> {
    //     if let Some(sled_db) = Sleder::open() {
    //         if let Ok(Some(bytes)) = sled_db.get("block-tail") {
    //             return Some(util::coder::deserialize(&bytes));
    //         }
    //     }
    //     None
    // }

    fn save_blockchain(blockchain: &BlockChain) {
        if let Some(sled_db) = Sleder::open() {
            sled_db
                .insert("blockchain", util::coder::serialize(blockchain))
                .unwrap();
        }
    }
}

#[test]
fn test() {
    // println!("{:?}", BlockChain::new());
    Sleder::show_all();
}
