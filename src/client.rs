/// Client is for calling EtherScan API
pub struct Client {
    pub key: String, // Api Key
    pub web: reqwest::Client, // Reqwest Client
}

impl Client {
    /// Creates a new `escan::Client` to allow you call Etherscan API
    pub fn new(api_key: &str) -> Self {
        Self {
            key: String::from(api_key),
            web: reqwest::Client::new(),
        }
    }
}