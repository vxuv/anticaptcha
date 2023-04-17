
mod client {
    #[cfg(feature = "blocking")]
    pub mod blocking;

    pub mod r#async;
}

mod resources;
