use sark::http::{Request, Response};
use http::{Method, StatusCode, HeaderValue};

#[monoio::test]
async fn test_request_creation() {
    let req = Request::new(Method::GET, "/test".parse().unwrap());
    
    assert_eq!(req.method(), &Method::GET);
    assert_eq!(req.uri().path(), "/test");
    
    let mut req = Request::new(Method::POST, "/api".parse().unwrap());
    req.headers_mut().insert("content-type", HeaderValue::from_static("application/json"));
    
    assert_eq!(req.headers().get("content-type").unwrap(), "application/json");
}

#[monoio::test]
async fn test_response_creation() {
    let resp = Response::ok();
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let mut resp = Response::new(StatusCode::CREATED);
    resp.set_body_str("Created resource");
    
    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(resp.body().to_vec(), b"Created resource");
    
    let mut resp = Response::ok();
    resp.headers_mut().insert("content-type", HeaderValue::from_static("text/plain"));
    
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/plain");
}