use std::io::Result as IoResult;
use monoio::net::{TcpListener, TcpStream};
use monoio::io::{AsyncReadRent, AsyncWriteRent};
use bytes::{BytesMut, BufMut};
use httparse::{Request as HttpParseRequest, EMPTY_HEADER, Status};
use http::{Method, Uri, Version, HeaderName, HeaderValue};

use crate::{
    app::App,
    error::Result,
    http::{Request, Response},
};

const MAX_HEADERS: usize = 100;
const BUFFER_SIZE: usize = 1048576;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn bind(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
        }
    }
    
    pub async fn serve<State, S: crate::service::Service<State>>(self, app: &App<State, S>) -> Result<()> {
        let listener = TcpListener::bind(self.addr.clone())?;
        tracing::info!("Server running on {}", self.addr);
        
        loop {
            let (stream, addr) = listener.accept().await?;
            tracing::debug!("Accepted connection from {}", addr);
            
            match handle_connection(stream, app).await {
                Ok(_) => (),
                Err(e) => tracing::error!("Error handling connection: {}", e),
            }
        }
    }
}

async fn handle_connection<State, S: crate::service::Service<State>>(mut stream: TcpStream, app: &App<State, S>) -> Result<()> {
    let request = read_http_request(&mut stream).await?;
    let response = app.handle(request).await?;
    write_response(stream, response).await?;
    Ok(())
}

async fn read_http_request(stream: &mut TcpStream) -> Result<Request> {
    let mut buffer = BytesMut::with_capacity(BUFFER_SIZE);
    buffer.resize(BUFFER_SIZE, 0);
    
    let (result, buf) = stream.read(buffer).await;
    buffer = buf;
    let n = result?;
    buffer.truncate(n);
    
    let mut headers = [EMPTY_HEADER; MAX_HEADERS];
    let mut req = HttpParseRequest::new(&mut headers);
    let status = req.parse(&buffer)?;
    
    match status {
        Status::Complete(size) => convert_request(req, &buffer[size..]),
        Status::Partial => {
            tracing::warn!("Incomplete HTTP request received");
            Err(crate::error::Error::BadRequest("Incomplete HTTP request".into()))
        }
    }
}

fn convert_request(req: HttpParseRequest, body: &[u8]) -> Result<Request> {
    let method = req.method
        .ok_or_else(|| crate::error::Error::BadRequest("Missing method".into()))
        .and_then(|m| Method::from_bytes(m.as_bytes())
            .map_err(|_| crate::error::Error::BadRequest("Invalid method".into())))?;
    
    let uri = req.path
        .ok_or_else(|| crate::error::Error::BadRequest("Missing URI".into()))
        .and_then(|p| Uri::try_from(p)
            .map_err(|_| crate::error::Error::BadRequest("Invalid URI".into())))?;
    
    let version = match req.version {
        Some(0) => Version::HTTP_10,
        Some(1) => Version::HTTP_11,
        _ => Version::HTTP_11,
    };
    
    let mut request = Request::new(method, uri);
    request.set_version(version);
    
    for header in req.headers.iter().filter(|h| !h.name.is_empty() && !h.value.is_empty()) {
        let name = HeaderName::from_bytes(header.name.as_bytes())
            .map_err(|_| crate::error::Error::BadRequest(
                format!("Invalid header name: {}", header.name)
            ))?;
            
        let value = HeaderValue::from_bytes(header.value)
            .map_err(|_| crate::error::Error::BadRequest(
                format!("Invalid header value for: {}", header.name)
            ))?;
            
        request.headers_mut().insert(name, value);
    }
    
    request.set_body(BytesMut::from(body));
    
    Ok(request)
}

async fn write_response(mut stream: TcpStream, response: Response) -> IoResult<()> {
    let status = response.status();
    let headers = response.headers();
    let body = response.body();
    
    let mut res = BytesMut::new();
    
    res.put_slice(format!(
        "HTTP/1.1 {} {}\r\n", 
        status.as_u16(), 
        status.canonical_reason().unwrap_or("")
    ).as_bytes());
    
    for (name, value) in headers.iter() {
        if let Ok(value_str) = value.to_str() {
            res.put_slice(format!("{}: {}\r\n", name, value_str).as_bytes());
        }
    }
    
    if !headers.contains_key("content-length") {
        res.put_slice(format!("Content-Length: {}\r\n", body.len()).as_bytes());
    }
    
    res.put_slice(b"\r\n");
    res.put_slice(body.as_ref());
    
    let (result, _) = stream.write(res).await;
    result.map(|_| ())
}