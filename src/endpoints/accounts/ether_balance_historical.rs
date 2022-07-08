use std::error::Error;

use crate::client::Client;
use crate::response::Response;

impl Client {
    /// Gets ether balance for a single address at a specific block
    pub async fn ether_balance_historical(&self, address: &str, block_no: u32) -> Result<String, Box<dyn Error>> {
        // Builds the URL
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=balancehistory&address={}&blockno={}&apikey={}",
            address,
            block_no,
            self.key,
        );
        // Returns
        Ok(self.web.get(url).send().await?.json::<Response<String>>().await?.result)
    }
}
