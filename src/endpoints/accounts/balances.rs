use std::error::Error;
use serde::Deserialize;

use crate::client::Client;
use crate::enums::Tag;

impl Client {
    /// Gets native cryptocurrency balance for multiple addresses
    pub async fn balances(&self, addresses: Vec<&str>, tag: Tag) -> Result<Vec<AccountAndBalance>, Box<dyn Error>> {
        // Builds the path
        let path = format!(
            "?module=account&action=balancemulti&address={}&tag={}",
            addresses.join(","),
            tag.to_str(),
        );
        // Returns
        self.request(path).await
    }
}



// Custom struct
#[derive(Debug, Deserialize)]
pub struct AccountAndBalance {
    pub account: String,
    pub balance: String,
}