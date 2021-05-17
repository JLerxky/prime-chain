use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Tile {
    pub point: (i64, i64, i64),
    pub tile_type: u8,
}
