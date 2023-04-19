/**
 * Captcha Solving Rust Library
 */
pub mod errors;
pub mod resources;
pub use crate::errors::*;
pub use crate::resources::*;

pub mod client {
    pub mod r#async;
}

pub mod config {
    pub type Client = crate::client::r#async::Client;
    pub type ClientError = crate::errors::ClientError;

}

pub use config::Client;
