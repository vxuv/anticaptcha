use crate::client::r#async::Client;

#[derive(Debug)]
pub struct Balance {
    status: u8,
    request: String,
    balance: f32,
}

pub struct GetBalance<'a> {
    client: &'a Client,
}

impl<'a> GetBalance<'a> {
    pub fn new(client: &'a Client) -> GetBalance {
        GetBalance { client }
    }

    pub fn execute(&self) -> Result<Balance, reqwest::Error> {
        let response = self.client.get("res.php?key={}&action=getbalance")?;
        let balance: Balance = response.json()?;
        Ok(balance)
    }
}