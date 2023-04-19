use std::fmt::{Display, Formatter};
pub enum ReCaptcha {
    V2,
    V3,
}

pub enum GeeTest {
    V1,
    V4
}

pub enum CaptchaTypes {
    NormalCaptcha,
    TextCaptcha,
    ReCaptcha,
    RotateCaptcha,
    FunCaptcha,
    KeyCaptcha,
    GeeTest(GeeTest),
    Hcaptcha,
    YandexCaptcha,
    CapyPuzzle,
    CloudflareTurnstile
}
impl Display for CaptchaTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CaptchaTypes::NormalCaptcha => write!(f, "normal"),
            CaptchaTypes::TextCaptcha => write!(f, "text"),
            CaptchaTypes::ReCaptcha => write!(f, "userrecaptcha"),
            CaptchaTypes::RotateCaptcha => write!(f, "rotate"),
            CaptchaTypes::FunCaptcha => write!(f, "funcaptcha"),
            CaptchaTypes::KeyCaptcha => write!(f, "keycaptcha"),
            CaptchaTypes::GeeTest(geetest) => match geetest {
                GeeTest::V1 => write!(f, "geetest"),
                GeeTest::V4 => write!(f, "geetest_v4"),
            },
            CaptchaTypes::Hcaptcha => write!(f, "hcaptcha"),
            CaptchaTypes::YandexCaptcha => write!(f, "yandex"),
            CaptchaTypes::CapyPuzzle => write!(f, "capy"),
            CaptchaTypes::CloudflareTurnstile => write!(f, "turnstile"),
        }
    }
}
/*
impl CaptchaTypes {
    pub fn to_string(&self) -> String {
        match self {
            CaptchaTypes::NormalCaptcha => "normal",
            CaptchaTypes::TextCaptcha => "text",
            CaptchaTypes::ReCaptcha => "userrecaptcha",
            CaptchaTypes::RotateCaptcha => "rotate",
            CaptchaTypes::FunCaptcha => "funcaptcha",
            CaptchaTypes::KeyCaptcha => "keycaptcha",
            CaptchaTypes::GeeTest(geetest) => match geetest {
                GeeTest::V1 => "geetest",
                GeeTest::V4 => "geetest_v4",
            },
            CaptchaTypes::Hcaptcha => "hcaptcha",
            CaptchaTypes::YandexCaptcha => "yandex",
            CaptchaTypes::CapyPuzzle => "capy",
            CaptchaTypes::CloudflareTurnstile => "turnstile",
        }.to_string()
    }
}
*/
pub enum ApiSubmissionErrors {
    WrongApiKey,
    KeyDoesNotExist,
    ZeroBalance,
    MissingPageUrl,
    NoSlotAvailable,
    ZeroCaptchaFilesize,
    TooBigCaptchaFilesize,
    WrongFileExtension,
    ImageTypeNotSupported,
    UploadError,
    IpNotAllowed,
    IpBanned,
    InvalidRecaptcha,
    BadGoogleKey,
    InvalidProxyFormat,
    MissingGoogleKey,
    CaptchaImageBlocked,
    TooManyBadImages,
    AccountRatelimit,
    AccountSuspended,
    BadParameters,
    BadProxy
}

