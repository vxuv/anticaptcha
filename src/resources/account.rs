use crate::{Client, client, errors::ClientError};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    status: u8,
    request: String,
    balance: f32,
}

#[derive(Clone, Debug)]
pub struct Account  {
    
}

impl Account {
    pub async fn get_balance(client: &Client) -> Result<Balance, ClientError> {
        let response = client.get("res.php?action=getbalance").await?;
        let text = response.text().await?;
        let balance: Balance = serde_json::from_str(&text)?;
        Ok(balance)
        
    }
}
