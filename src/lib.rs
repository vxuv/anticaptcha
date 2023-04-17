/**
 * AntiCaptcha API Client
 */
mod resources;
pub use crate::resources::*;

pub mod client {
    #[cfg(feature = "blocking")]
    pub mod blocking;

    pub mod r#async;
}


pub mod config {
    pub type Client = crate::client::r#async::Client;
}

pub use config::Client;
