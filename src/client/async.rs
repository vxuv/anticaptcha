use reqwest::Client as ReqwestClient;
use crate::{errors::{ApiErrors, ClientError}, types::CaptchaTypes};

#[derive(Debug)]
pub struct Client {
    host: String,
    pub api_key: String,
    reqwest_client: ReqwestClient,
}

pub enum InAction {
    SubmitCaptcha(CaptchaTypes)
}

pub enum ResAction {
    GetBalance,
    GetCaptcha
}

pub enum Action {
    In(InAction),
    Res(ResAction)
}

impl Action {
    fn to_string(&self) -> String {
        match self {
            Action::In(action) => match action {
                InAction::SubmitCaptcha(captcha_type) => format!("in.php?action=upload&captchaType={}", captcha_type.to_string()),
            },
            Action::Res(action) => match action {
                ResAction::GetBalance => "res.php?action=getbalance".to_string(),
                ResAction::GetCaptcha => "res.php?action=getcaptcha".to_string(),
            }
        }
    }
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


    pub async fn get(&self, action: Action) -> Result<reqwest::Response, ClientError> {
        let url = self.build_url(action);
        let response = self.reqwest_client.get(&url).send().await;
        match response {
            Ok(response) => Ok(response),
            Err(error) => Err(ClientError::Reqwest(error)),
        }
        
    }

    pub async fn post(&self, action: Action) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.build_url(action);
        let response = self.reqwest_client.post(&url).send().await;
        match response {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }

    fn build_url(&self, action: Action) -> String {
        format!("{}{}&key={}&json=1", self.host, action.to_string(), self.api_key)
    }

}