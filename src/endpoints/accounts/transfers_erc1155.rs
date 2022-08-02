use std::error::Error;
use serde::Deserialize;

use crate::client::Client;
use crate::enums::Sort;

impl Client {
    /// Gets a list of ERC1155 token transfers by address 
    /// !!!Note: Only supported on Ethereum Chain
    pub async fn transfers_erc1155(&self, contract_address: &str, address: &str, page: u32, offset: u32, start_block: u32, end_block: u32, sort: Sort) -> Result<Vec<Transfer>, Box<dyn Error>> {
        // Builds the path
        let path = format!(
            "?module=account&action=token1155tx&contractaddress={}&address={}&page={}&offset={}&startblock={}&endblock={}&sort={}",
            contract_address,
            address,
            page,
            offset,
            start_block,
            end_block,
            sort.to_str(),
        );
        // Returns
        self.request(path).await
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