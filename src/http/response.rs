use bytes::BytesMut;
use http::{StatusCode, HeaderMap};

#[derive(Clone)]
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: BytesMut,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            body: BytesMut::new(),
        }
    }

    pub fn ok() -> Self { Self::new(StatusCode::OK) }
    pub fn not_found() -> Self { Self::new(StatusCode::NOT_FOUND) }
    pub fn status(&self) -> StatusCode { self.status }
    pub fn set_status(&mut self, status: StatusCode) { self.status = status; }
    pub fn headers(&self) -> &HeaderMap { &self.headers }
    pub fn headers_mut(&mut self) -> &mut HeaderMap { &mut self.headers }
    pub fn body(&self) -> &BytesMut { &self.body }
    pub fn body_mut(&mut self) -> &mut BytesMut { &mut self.body }
    
    pub fn set_body(&mut self, body: impl Into<BytesMut>) {
        self.body = body.into();
    }
    
    pub fn set_body_str(&mut self, body: &str) -> &mut Self {
        self.body = BytesMut::from(body.as_bytes());
        self
    }
    
    pub fn body_str(&self) -> Option<&str> {
        std::str::from_utf8(self.body.as_ref()).ok()
    }
}