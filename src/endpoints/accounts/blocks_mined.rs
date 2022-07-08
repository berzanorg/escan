use std::error::Error;
use serde::Deserialize;

use crate::enums::BlockType;
use crate::client::Client;
use crate::response::Response;


impl Client {
    /// Gets a list of blocks mined by address
    pub async fn blocks_mined(&self, address: &str, block_type: BlockType, page: u32, offset: u32) -> Result<Vec<Block>, Box<dyn Error>> {
        // Builds the URL
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=getminedblocks&address={}&blocktype={}&page={}&offset={}&apikey={}",
            address,
            block_type.to_str(),
            page,
            offset,
            self.key,
        );
        // Returns
        Ok(self.web.get(url).send().await?.json::<Response<Vec<Block>>>().await?.result)
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