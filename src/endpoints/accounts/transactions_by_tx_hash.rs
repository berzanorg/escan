use std::error::Error;
use serde::Deserialize;

use crate::client::Client;


impl Client {
    /// Gets a list of internal transactions performed within a transaction
    pub async fn transactions_by_tx_hash(&self, tx_hash: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
        // Builds the path
        let path = format!(
            "?module=account&action=txlistinternal&txhash={}",
            tx_hash,
        );
        // Returns
        self.request(path).await
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
