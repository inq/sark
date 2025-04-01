use crate::{
    http::{Request, Response},
    error::{Result, Error},
    service::Service,
};
use http::Method;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct Router<S, N = Nil> {
    node: N,
    _marker: PhantomData<S>,
}

pub struct Nil;

impl<S> Router<S, Nil> {
    pub fn new() -> Self {
        Self {
            node: Nil,
            _marker: PhantomData,
        }
    }
}

impl<S, N> Router<S, N> {
    pub fn route<H>(self, method: Method, path: &str, handler: H) -> Router<S, Node<H, N>>
    where
        H: Service<S>,
    {
        Router {
            node: Node {
                method,
                path: path.to_string(),
                handler,
                next: self.node,
            },
            _marker: PhantomData,
        }
    }
}

impl<S, N> Service<S> for Router<S, N> 
where 
    N: RouteNode<S>,
{
    async fn call(&self, req: Request, state: &S) -> Result<Response> {
        self.node.match_route(req, state).await
    }
}

pub trait RouteNode<S> {
    async fn match_route(&self, req: Request, state: &S) -> Result<Response>;
}

pub struct Node<H, N> {
    method: Method,
    path: String,
    handler: H,
    next: N,
}

impl<S> RouteNode<S> for Nil {
    async fn match_route(&self, _req: Request, _state: &S) -> Result<Response> {
        Err(Error::NotFound)
    }
}

impl<S, H, N> RouteNode<S> for Node<H, N>
where
    H: Service<S>,
    N: RouteNode<S>,
{
    async fn match_route(&self, req: Request, state: &S) -> Result<Response> {
        let method = req.method();
        let path = req.uri().path().to_string();
        
        if *method == self.method {
            if !self.path.contains(':') {
                if self.path == path {
                    return self.handler.call(req, state).await;
                }
            } else {
                let mut req_clone = req.clone();
                if extract_path_match(&self.path, &path, &mut req_clone) {
                    return self.handler.call(req_clone, state).await;
                }
            }
        }
        
        self.next.match_route(req, state).await
    }
}

fn extract_path_match(route_path: &str, req_path: &str, req: &mut Request) -> bool {
    let route_segments: Vec<&str> = route_path.split('/').filter(|s| !s.is_empty()).collect();
    let req_segments: Vec<&str> = req_path.split('/').filter(|s| !s.is_empty()).collect();
    
    if route_segments.len() != req_segments.len() {
        return false;
    }
    
    let mut params = HashMap::new();
    for (i, segment) in route_segments.iter().enumerate() {
        if segment.starts_with(':') {
            let param_name = &segment[1..];
            params.insert(param_name.to_string(), req_segments[i].to_string());
        } else if *segment != req_segments[i] {
            return false;
        }
    }
    
    if !params.is_empty() {
        req.set_path_params(params);
    }
    
    true
}