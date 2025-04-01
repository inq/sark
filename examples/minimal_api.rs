use http::Method;
use sark::{
    app::App,
    server::Server,
    service::{Service, Router},
    http::{Request, Response},
    error::Result,
};

struct HelloService;

impl Service for HelloService {
    async fn call(&self, _req: Request, _state: &()) -> Result<Response> {
        let mut resp = Response::ok();
        resp.set_body_str("Hello, World!");
        Ok(resp)
    }
}

struct GreetService;

impl Service for GreetService {
    async fn call(&self, req: Request, _state: &()) -> Result<Response> {
        let path_name = req.path_param("name").map(|s| s.to_string());
        let query_name = req.query("name").map(|s| s.to_string());
        
        let name = path_name
            .or(query_name)
            .unwrap_or_else(|| "Guest".to_string());

        let mut resp = Response::ok();
        resp.set_body_str(&format!("Hello, {}!", name));
        Ok(resp)
    }
}

struct CustomService;

impl Service for CustomService {
    async fn call(&self, _req: Request, _state: &()) -> Result<Response> {
        let mut resp = Response::ok();
        resp.set_body_str("Greetings from custom handler!");
        Ok(resp)
    }
}

type AppState = String;

struct StateService;

impl Service<AppState> for StateService {
    async fn call(&self, _req: Request, state: &AppState) -> Result<Response> {
        let mut resp = Response::ok();
        resp.set_body_str(&format!("Service says: {}", state));
        Ok(resp)
    }
}

fn main() -> Result<()> {
    let app = App::default()
        .route(Method::GET, "/", HelloService)
        .route(Method::GET, "/greet/:name", GreetService)
        .route(Method::GET, "/custom", CustomService);

    let _custom_state_app = App::new(Router::new(), "Hello from state!".to_string())
        .route(Method::GET, "/state", StateService);

    let mut runtime = monoio::RuntimeBuilder::<monoio::LegacyDriver>::new()
        .build()
        .unwrap();

    runtime.block_on(async {
        Server::bind("127.0.0.1:3000")
            .serve(&app)
            .await
    })
}