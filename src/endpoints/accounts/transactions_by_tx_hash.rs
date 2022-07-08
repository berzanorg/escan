use std::error::Error;
use serde::Deserialize;

use crate::client::Client;
use crate::response::Response;


impl Client {
    /// Gets a list of internal transactions performed within a transaction
    pub async fn transactions_by_tx_hash(&self, tx_hash: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
        // Builds the URL
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=txlistinternal&txhash={}&apikey={}",
            tx_hash,
            self.key,
        );
        // Returns
        Ok(self.web.get(url).send().await?.json::<Response<Vec<Transaction>>>().await?.result)
    }
}


// Custom struct
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub blockNumber: String,
    pub timeStamp: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub contractAddress: String,
    pub input: String,
    pub r#type: String,
    pub gas: String,
    pub gasUsed: String,
    pub isError: String,
    pub errCode: String,
}
