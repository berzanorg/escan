use std::error::Error;
use crate::client::Client;

impl Client {
    /// Gets native cryptocurrency balance for a single address at a specific block
    pub async fn balance_historical(&self, address: &str, block_no: u32) -> Result<String, Box<dyn Error>> {
        // Builds the path
        let path = format!(
            "?module=account&action=balancehistory&address={}&blockno={}",
            address,
            block_no,
        );
        // Returns
        self.request(path).await
    }
}
