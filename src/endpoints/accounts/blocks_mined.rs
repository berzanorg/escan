use std::error::Error;
use serde::Deserialize;

use crate::enums::BlockType;
use crate::client::Client;


impl Client {
    /// Gets a list of blocks mined by address
    pub async fn blocks_mined(&self, address: &str, block_type: BlockType, page: u32, offset: u32) -> Result<Vec<Block>, Box<dyn Error>> {
        // Builds the path
        let path = format!(
            "?module=account&action=getminedblocks&address={}&blocktype={}&page={}&offset={}",
            address,
            block_type.to_str(),
            page,
            offset,
        );
        // Return the data
        self.request(path).await
    }
}



// Custom struct
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Block {
    pub blockNumber: String,
    pub timeStamp: String,
    pub blockReward: String,
}