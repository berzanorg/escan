use std::error::Error;
use serde::Deserialize;

use crate::client::Client;
use crate::enums::Sort;


impl Client {
    /// Gets a list of internal transactions performed within a block range
    pub async fn transactions_by_range(&self, start_block: u32, end_block: u32, page: u32, offset: u32, sort: Sort) -> Result< Vec<Transaction>, Box<dyn Error>> {
        // Builds the path
        let path = format!(
            "?module=account&action=txlistinternal&startblock={}&endblock={}&page={}&offset={}&sort={}",
            start_block,
            end_block,
            page,
            offset,
            sort.to_str(),
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
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub contractAddress: String,
    pub input: String,
    pub r#type: String,
    pub gas: String,
    pub gasUsed: String,
    pub traceId: String,
    pub isError: String,
    pub errCode: String,
}