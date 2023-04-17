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

    pub fn from_url(host: impl String, api_key: impl Into<String>) -> Client {
        Client {
            host,
            api_key,
            reqwest_client: ReqwestClient::blocking::new(),
        }
    }

    pub fn get(&self, path: impl String) -> Result<reqwest::Response, reqwest::Error> {
        self.reqwest_client.get(&format!("{}{}", self.host, path)).send()
    }

    pub fn post(&self, path: impl String) -> Result<reqwest::Response, reqwest::Error> {
        self.reqwest_client.post(&format!("{}{}", self.host, path)).send()
    }

}