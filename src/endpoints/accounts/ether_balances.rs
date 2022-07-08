use std::error::Error;
use serde::Deserialize;

use crate::client::Client;
use crate::response::Response;
use crate::enums::Tag;

impl Client {
    /// Gets ether balance for multiple addresses
    pub async fn ether_balances(&self, addresses: Vec<&str>, tag: Tag) -> Result<Vec<AccountAndBalance>, Box<dyn Error>> {
        // Builds the URL
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=balancemulti&address={}&tag={}&apikey={}",
            addresses.join(","),
            tag.to_str(),
            self.key,
        );
        // Returns
        Ok(self.web.get(url).send().await?.json::<Response<Vec<AccountAndBalance>>>().await?.result)
    }
}



// Custom struct
#[derive(Debug, Deserialize)]
pub struct AccountAndBalance {
    pub account: String,
    pub balance: String,
}