use std::error::Error;

use serde::de::DeserializeOwned;

use crate::{Chain, response::Response};

/// Client is for calling EtherScan API
pub struct Client {
    key: String, // Api Key
    web: reqwest::Client, // Reqwest Client
    base_url: String, 
}

impl Client {
    /// Creates a new `escan::Client` to allow you call Etherscan API
    /// Provide an API key and a `Chain`
    pub fn new(api_key: &str, chain: Chain) -> Self {
        Self {
            key: String::from(api_key),
            web: reqwest::Client::new(),
            base_url: chain.to_str().to_string(),
        }
    }

    pub async fn request<T>(&self, path: String) -> Result<T, Box<dyn Error>>
    where T: DeserializeOwned 
    {
        let url = format!("{}{}&apikey={}", self.base_url, path, self.key);
        Ok(
            self.web.get(url)
                .send()
                .await?
                .json::<Response<T>>()
                .await?
                .result
        )
    }
}