use reqwest::Client as ReqwestClient;
use crate::errors::{ApiErrors, ClientError};

#[derive(Debug)]
pub struct Client {
    host: String,
    pub api_key: String,
    reqwest_client: ReqwestClient,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Client {
        Client::from_url("https://2captcha.com/", api_key)
    }

    pub fn from_url(host: impl Into<String>, api_key: impl Into<String>) -> Client {
        Client {
            host: host.into(),
            api_key: api_key.into(),
            reqwest_client: ReqwestClient::new(),
        }
    }

    fn format_url(&self, path: String) -> String {
        format!("{}{}&key={}&json=1", self.host, path, self.api_key)
    }

    pub async fn get(&self, path: impl Into<String>) -> Result<reqwest::Response, ClientError> {
        let response = self.reqwest_client.get(&self.format_url(path.into())).send().await;
        match response {
            Ok(response) => Ok(response),
            Err(error) => Err(ClientError::Reqwest(error)),
        }
        
    }

    pub async fn post(&self, path: impl Into<String>) -> Result<reqwest::Response, reqwest::Error> {
        self.reqwest_client.post(&self.format_url(path.into())).send().await
    }

    /*
    pub fn parse_error(&self, response: reqwest::Response) -> Result<(), ClientError> {
        let error: ApiErrors = response.json().await?;
        Err(ClientError::Api(error))
    }
    */


}