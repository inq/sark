#![allow(async_fn_in_trait)]

pub mod app;
pub mod http;
pub mod error;
pub mod server;
pub mod service;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::service::{Service, Router};
    pub use crate::http::{Request, Response};
    pub use crate::error::{Error, Result};
    pub use crate::server::Server;
    pub use http::{Method, StatusCode};
}
