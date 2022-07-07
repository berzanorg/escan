use serde::{Deserialize, Serialize};

/// Response is the main struct for Etherscan API responses
#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub status: String,
    pub message: String,
    pub result: T,
}