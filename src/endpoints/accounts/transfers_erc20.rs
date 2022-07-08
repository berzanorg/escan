use std::error::Error;
use serde::Deserialize;


use crate::client::Client;
use crate::response::Response;
use crate::enums::Sort;

impl Client {
    /// Gets a list of ERC20 token transfers by address 
    pub async fn transfers_erc20(&self, contract_address: &str, address: &str, page: u32, offset: u32, start_block: u32, end_block: u32, sort: Sort) -> Result<Vec<Transfer>, Box<dyn Error>> {
        // Builds the URL
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page={}&offset={}&startblock={}&endblock={}&sort={}&apikey={}",
            contract_address,
            address,
            page,
            offset,
            start_block,
            end_block,
            sort.to_str(),
            self.key,
        );
        // Returns
        Ok(self.web.get(url).send().await?.json::<Response<Vec<Transfer>>>().await?.result)
    }
}



// Custom struct
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub blockNumber: String,
    pub timeStamp: String,
    pub hash: String,
    pub nonce: String,
    pub blockHash: String,
    pub from: String,
    pub contractAddress: String,
    pub to: String,
    pub value: String,
    pub tokenName: String,
    pub tokenSymbol: String,
    pub tokenDecimal: String,
    pub transactionIndex: String,
    pub gas: String,
    pub gasPrice: String,
    pub gasUsed: String,
    pub cumulativeGasUsed: String,
    pub input: String,
    pub confirmations: String,
}