use std::error::Error;
use serde::Deserialize;

use crate::client::Client;
use crate::response::Response;
use crate::enums::Sort;


impl Client {
    /// Gets a list of internal transactions by address 
    pub async fn transactions_internal(&self, address: &str, start_block: u32, end_block: u32, page: u32, offset: u32, sort: Sort) -> Result<Vec<Transaction>, Box<dyn Error>> {
        // Builds the URL
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&page={}&offset={}&sort={}&apikey={}",
            address,
            start_block,
            end_block,
            page,
            offset,
            sort.to_str(),
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