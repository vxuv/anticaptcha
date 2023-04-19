use crate::{Client, client, errors::ClientError};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::client::r#async::Action;


#[derive(Debug, Deserialize, Serialize)]
pub struct TempBalance {
    status: u8,
    request: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    status: u8,
    balance: f32
}

#[derive(Clone, Debug)]
pub struct Account  {
    
}

impl Account {
    pub async fn get_balance(client: &Client) -> Result<Balance, ClientError> {
        let response = client.get(Action::Res(client::r#async::ResAction::GetBalance)).await?;
        let balance = response.json::<TempBalance>().await?;
        Ok(Balance {
            status: balance.status,
            balance: balance.request.parse::<f32>().unwrap()
        })

    }
}
