use thiserror::Error;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    
    #[error("Method not allowed")]
    MethodNotAllowed,
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("HTTP parse error: {0}")]
    HttpParse(#[from] httparse::Error),
}