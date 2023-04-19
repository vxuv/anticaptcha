#[derive(Debug)]
pub enum ApiErrors {
    CaptchaNotReady,
    CaptchaUnsolvable,
    WrongApiKey,
    KeyDoesNotExist,
    WrongIdFormat,
    WrongCaptchaId,
    BadDuplicates,
    ReportNotRecorded,
    DuplicateReport,
    AccountSuspended,
    IpNOtAllowed,
    TokenExpired,
    MissingAction,
    BadProxy
}

#[derive(Debug)]
pub enum ClientError {
    Reqwest(reqwest::Error),
    Api(ApiErrors),
    Json(serde_json::Error),
    Io(std::io::Error),
}


impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError::Reqwest(error)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(error: serde_json::Error) -> Self {
        ClientError::Json(error)
    }
}

impl From<std::io::Error> for ClientError {
    fn from(error: std::io::Error) -> Self {
        ClientError::Io(error)
    }
}

