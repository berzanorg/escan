use std::error::Error;

use crate::client::Client;
use crate::enums::Tag;


impl Client {
    /// Gets native cryptocurrency balance for a single address
    pub async fn balance(&self, address: &str, tag: Tag) -> Result<String, Box<dyn Error>> {
        // Builds the path
        let path = format!(
            "?module=account&action=balance&address={}&tag={}",
            address,
            tag.to_str(),
        );
        // Returns
        self.request(path).await
    }
}
