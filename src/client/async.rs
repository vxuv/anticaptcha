use reqwest::Client as ReqwestClient;

#[derive(Debug)]
pub struct Client {
    host: String,
    api_key: String,
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

    pub async fn get(&self, path: impl Into<String>) -> Result<reqwest::Response, reqwest::Error> {
        self.reqwest_client.get(&format!("{}{}", self.host, path)).send()
    }

    pub async fn post(&self, path: impl Into<String>) -> Result<reqwest::Response, reqwest::Error> {
        self.reqwest_client.post(&format!("{}{}", self.host, path)).send()
    }

}