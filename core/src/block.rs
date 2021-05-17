use crate::tile::Tile;
use serde::{Deserialize, Serialize};
use util::sleder::Sleder;
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: [u8; 32],
    pub body: [Tile; 27],
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockHeader {
    pub height: u64,
    pub time: i64,
    pub body_hash: [u8; 32],
    pub pre_hash: [u8; 32],
}

impl Block {
    /// 创建新区块
    pub fn new() -> Self {
        Block::default()
    }

    /// 创建创世区块
    pub fn new_genesis_block() -> Self {
        // body
        let body = [Tile {
            point: (0, 0, 0),
            tile_type: 0,
        }; 27];
        let mut body_hash: [u8; 32] = [0; 32];
        util::coder::get_hash(&util::coder::serialize(&body)[..], &mut body_hash);

        // header
        let header = BlockHeader {
            height: 1,
            time: chrono::Utc::now().timestamp(),
            body_hash,
            pre_hash: [0; 32],
        };
        let mut hash: [u8; 32] = [0; 32];
        util::coder::get_hash(&util::coder::serialize(&header)[..], &mut hash);

        let genesis_block = Block { header, hash, body };
        // 持久化创世块
        Block::save_block(&genesis_block);
        genesis_block
    }

    fn save_block(block: &Block) {
        if let Some(sled_db) = Sleder::open() {
            sled_db
                .insert(block.hash, util::coder::serialize(block))
                .unwrap();
        }
    }
}
