# SARK

**S**tatic **A**synchronous **R**ust web frame**K**it

## Overview

SARK is a lightweight, single-threaded asynchronous web framework for Rust, built on top of the monoio runtime. It uses Rust's type system to provide fully static routing with zero dynamic dispatch.

## Features

- Zero dynamic dispatch - fully static routing using Rust's type system
- No `Box<dyn>` or trait objects - maximized compiler optimizations
- Single-threaded async architecture (no `Send`/`Sync` constraints)
- Type-safe request routing and parameter extraction
- State management via generics
- Minimal API surface - easy to learn and use
- Async/await native with AFIT (Async Functions in Traits)

## Example

```rust
use http::Method;
use sark::{
    app::App,
    server::Server,
    service::Service,
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

fn main() -> Result<()> {
    let app = App::default()
        .route(Method::GET, "/", HelloService);

    let mut runtime = monoio::RuntimeBuilder::<monoio::LegacyDriver>::new()
        .build()
        .unwrap();

    runtime.block_on(async {
        Server::bind("127.0.0.1:3000")
            .serve(&app)
            .await
    })
}
```

## Development Status

SARK is currently in early development and is not yet recommended for production use.

## License

MIT