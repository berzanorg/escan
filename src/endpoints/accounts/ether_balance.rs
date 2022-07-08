use std::error::Error;

use crate::client::Client;
use crate::response::Response;
use crate::enums::Tag;


impl Client {
    /// Gets ether balance for a single address
    pub async fn ether_balance(&self, address: &str, tag: Tag) -> Result<String, Box<dyn Error>> {
        // Builds the URL
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=balance&address={}&tag={}&apikey={}",
            address,
            tag.to_str(),
            self.key,
        );
        // Returns
        Ok(self.web.get(url).send().await?.json::<Response<String>>().await?.result)
    }
}
