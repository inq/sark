use crate::{
    http::{Request, Response},
    error::Result,
    service::{Service, Router, Node},
};
use http;

pub struct App<State, R = Router<State>> {
    service: R,
    state: State,
}

impl<State, R: Service<State>> App<State, R> {
    pub fn new(service: R, state: State) -> Self {
        Self {
            service,
            state,
        }
    }

    pub async fn handle(&self, req: Request) -> Result<Response> {
        self.service.call(req, &self.state).await
    }

    pub fn service(&self) -> &R {
        &self.service
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

impl<R: Service<()>> App<(), R> {
    pub fn with_empty_state(service: R) -> Self {
        Self {
            service,
            state: (),
        }
    }
}

impl Default for App<()> {
    fn default() -> Self {
        Self::with_empty_state(Router::new())
    }
}

impl<State, N> App<State, Router<State, N>> {
    pub fn route<H: Service<State>>(self, method: http::Method, path: &str, handler: H) -> App<State, Router<State, Node<H, N>>> {
        App {
            service: self.service.route(method, path, handler),
            state: self.state,
        }
    }
}