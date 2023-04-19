
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
    GeeTest,
    Hcaptcha,
    YandexCaptcha,
    CapyPuzzle,
    LeminCaptcha,
    CloudflareTurnstile,
    AmazonWaf
}

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

