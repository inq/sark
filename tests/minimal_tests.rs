#[monoio::test]
async fn test_http_response() {
    use sark::http::Response;
    use http::StatusCode;
    use bytes::BytesMut;

    let mut resp = Response::ok();
    assert_eq!(resp.status(), StatusCode::OK);

    resp.set_body_str("Hello, World!");
    assert_eq!(resp.body(), &BytesMut::from("Hello, World!".as_bytes()));
}

#[monoio::test]
async fn test_http_request() {
    use sark::http::Request;
    use http::{Method, HeaderValue, Uri};
    use std::collections::HashMap;

    let req = Request::new(Method::GET, Uri::from_static("/test"));
    assert_eq!(req.method(), &Method::GET);
    assert_eq!(req.uri().path(), "/test");

    let mut req = Request::new(Method::POST, Uri::from_static("/api"));
    req.headers_mut().insert("content-type", HeaderValue::from_static("application/json"));
    assert_eq!(req.headers().get("content-type").unwrap(), "application/json");

    let mut req = Request::new(Method::GET, Uri::from_static("/users/123"));
    let mut params = HashMap::new();
    params.insert("id".to_string(), "123".to_string());
    req.set_path_params(params);
    assert_eq!(req.path_param("id"), Some("123"));
}

#[monoio::test]
async fn test_service_call() {
    use sark::{
        http::{Request, Response},
        service::Service,
        error::Result,
    };
    use http::{Method, Uri};
    use bytes::BytesMut;

    #[derive(Clone)]
    struct TestHandler;

    impl Service for TestHandler {
        async fn call(&self, _req: Request, _state: &()) -> Result<Response> {
            let mut res = Response::ok();
            res.set_body_str("Hello, World!");
            Ok(res)
        }
    }

    let handler = TestHandler;
    let req = Request::new(Method::GET, Uri::from_static("/"));
    let res = handler.call(req, &()).await.unwrap();
    
    assert_eq!(res.status(), http::StatusCode::OK);
    assert_eq!(res.body(), &BytesMut::from("Hello, World!".as_bytes()));
}

#[monoio::test]
async fn test_router() {
    use sark::{
        http::{Request, Response},
        service::Service,
        error::Result,
        app::App,
    };
    use http::{Method, Uri};
    use bytes::BytesMut;

    #[derive(Clone)]
    struct HelloHandler;

    impl Service for HelloHandler {
        async fn call(&self, _req: Request, _state: &()) -> Result<Response> {
            let mut res = Response::ok();
            res.set_body_str("Hello, World!");
            Ok(res)
        }
    }

    #[derive(Clone)]
    struct EchoHandler;

    impl Service for EchoHandler {
        async fn call(&self, req: Request, _state: &()) -> Result<Response> {
            let name = req.path_param("name").unwrap_or("Guest");
            let mut res = Response::ok();
            res.set_body_str(&format!("Hello, {}!", name));
            Ok(res)
        }
    }

    let app = App::default()
        .route(Method::GET, "/", HelloHandler)
        .route(Method::GET, "/echo/:name", EchoHandler);

    let req = Request::new(Method::GET, Uri::from_static("/"));
    let res = app.handle(req).await.unwrap();
    assert_eq!(res.status(), http::StatusCode::OK);
    assert_eq!(res.body(), &BytesMut::from("Hello, World!".as_bytes()));

    let req = Request::new(Method::GET, Uri::from_static("/echo/Alice"));
    let res = app.handle(req).await.unwrap();
    assert_eq!(res.status(), http::StatusCode::OK);
    assert_eq!(res.body(), &BytesMut::from("Hello, Alice!".as_bytes()));
}