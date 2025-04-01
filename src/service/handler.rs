use crate::{
    http::{Request, Response},
    error::Result,
    service::Service,
};
use std::future::Future;
use std::marker::PhantomData;

pub struct FnHandler<F, Fut, State = ()>
where
    F: Fn(Request, &State) -> Fut,
    Fut: Future<Output = Result<Response>>,
{
    f: F,
    _fut: PhantomData<Fut>,
    _state: PhantomData<State>,
}

impl<F, Fut, State> FnHandler<F, Fut, State>
where
    F: Fn(Request, &State) -> Fut,
    Fut: Future<Output = Result<Response>>,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _fut: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<F, Fut, State> Service<State> for FnHandler<F, Fut, State>
where
    F: Fn(Request, &State) -> Fut,
    Fut: Future<Output = Result<Response>>,
{
    async fn call(&self, req: Request, state: &State) -> Result<Response> {
        (self.f)(req, state).await
    }
}

impl<F, Fut, State> Clone for FnHandler<F, Fut, State>
where
    F: Fn(Request, &State) -> Fut + Clone,
    Fut: Future<Output = Result<Response>>,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _fut: PhantomData,
            _state: PhantomData,
        }
    }
}