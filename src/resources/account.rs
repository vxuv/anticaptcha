use crate::Client;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    status: u8,
    request: String,
    balance: f32,
}

#[warn(dead_code)]
pub struct GetBalance<'a> {
    client: &'a Client,
}

impl<'a> GetBalance<'a> {
    pub fn new(client: &'a Client) -> GetBalance {
        GetBalance { client }
    }

    pub async fn execute(&self) -> Result<Balance, reqwest::Error> {
        let response: Response = self.client.get("res.php?key={}&action=getbalance").await?;
        let balance = response.json::<Balance>().await?;
        Ok(balance)
    }

}