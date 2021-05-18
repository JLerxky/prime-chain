use serde::{Deserialize, Serialize};

/// 区块
#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Block<T> {
    pub header: BlockHeader,
    pub hash: [u8; 32],
    pub body: T,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct BlockHeader {
    pub height: u64,
    pub time: i64,
    pub body_hash: [u8; 32],
    pub pre_hash: [u8; 32],
}

impl<T: Default + Serialize> Block<T> {
    /// 创建区块
    pub fn new(height: u64, body: T) -> Self {
        let mut body_hash: [u8; 32] = [0; 32];
        util::coder::get_hash(&util::coder::serialize(&body)[..], &mut body_hash);

        // header
        let header = BlockHeader {
            height,
            time: chrono::Utc::now().timestamp(),
            body_hash,
            pre_hash: [0; 32],
        };
        let mut hash: [u8; 32] = [0; 32];
        util::coder::get_hash(&util::coder::serialize(&header)[..], &mut hash);

        Block { header, hash, body }
    }
}
