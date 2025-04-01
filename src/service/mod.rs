mod router;
mod handler;

pub use router::{Router, Nil, Node, RouteNode};
pub use handler::FnHandler;

use crate::{
    http::{Request, Response},
    error::Result,
};

pub trait Service<State = ()> {
    async fn call(&self, req: Request, state: &State) -> Result<Response>;
}