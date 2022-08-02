use std::error::Error;
use serde::Deserialize;

use crate::client::Client;
use crate::enums::Sort;


impl Client {
    /// Gets a list of normal transactions by address 
    pub async fn transactions(&self, address: &str, start_block: u32, end_block: u32, page: u32, offset: u32, sort: Sort) -> Result<Vec<Transaction>, Box<dyn Error>> {
        // Builds the path
        let path = format!(
            "?module=account&action=txlist&address={}&startblock={}&endblock={}&page={}&offset={}&sort={}",
            address,
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
    pub nonce: String,
    pub blockHash: String,
    pub transactionIndex: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gasPrice: String,
    pub isError: String,
    pub txreceipt_status: String,
    pub input: String,
    pub contractAddress: String,
    pub cumulativeGasUsed: String,
    pub gasUsed: String,
    pub confirmations: String,
    pub methodId: String,
    pub functionName: String,
}