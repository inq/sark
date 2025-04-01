use std::collections::HashMap;
use bytes::BytesMut;
use http::{Method, Uri, Version, HeaderMap};
use serde_urlencoded;
use std::str::FromStr;

pub struct Request {
    method: Method,
    uri: Uri,
    version: Version,
    headers: HeaderMap,
    body: BytesMut,
    path_params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: Method, uri: Uri) -> Self {
        Self {
            method,
            uri,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            body: BytesMut::new(),
            path_params: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        Self::new(Method::GET, Uri::from_static("/"))
    }

    pub fn method(&self) -> &Method { &self.method }
    pub fn uri(&self) -> &Uri { &self.uri }
    pub fn version(&self) -> Version { self.version }
    pub fn set_version(&mut self, version: Version) { self.version = version; }
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

    pub fn path_param<T: AsRef<str>>(&self, key: T) -> Option<&str> {
        self.path_params.get(key.as_ref()).map(|s| s.as_str())
    }

    pub fn path_params(&self) -> &HashMap<String, String> {
        &self.path_params
    }

    pub fn set_path_params(&mut self, params: HashMap<String, String>) -> &mut Self {
        self.path_params = params;
        self
    }

    pub fn insert_path_param(&mut self, key: impl AsRef<str>, value: impl Into<String>) -> &mut Self {
        self.path_params.insert(key.as_ref().to_string(), value.into());
        self
    }

    pub fn query<T: AsRef<str>>(&self, key: T) -> Option<String> {
        self.uri.query().and_then(|q| {
            let params = serde_urlencoded::from_str::<HashMap<String, String>>(q).ok()?;
            params.get(key.as_ref()).map(|s| s.to_owned())
        })
    }

    pub fn query_params(&self) -> Option<HashMap<String, String>> {
        self.uri.query().and_then(|q| {
            serde_urlencoded::from_str::<HashMap<String, String>>(q).ok()
        })
    }

    pub fn with_uri(&mut self, uri: Uri) -> &mut Self {
        self.uri = uri;
        self
    }

    pub fn with_query<T: serde::Serialize>(&mut self, query: &T) -> Result<&mut Self, serde_urlencoded::ser::Error> {
        let query_string = serde_urlencoded::to_string(query)?;
        let current_uri = self.uri.clone();
        
        let mut parts = current_uri.into_parts();
        parts.path_and_query = Some(
            match parts.path_and_query {
                Some(path_and_query) => {
                    let path = path_and_query.path();
                    let new_path_and_query = format!("{}?{}", path, query_string);
                    http::uri::PathAndQuery::from_str(&new_path_and_query)
                        .expect("valid path and query")
                },
                None => {
                    let new_path_and_query = format!("/?{}", query_string);
                    http::uri::PathAndQuery::from_str(&new_path_and_query)
                        .expect("valid path and query")
                }
            }
        );
        
        let new_uri = Uri::from_parts(parts)
            .expect("valid URI parts");
        
        self.uri = new_uri;
        Ok(self)
    }
}

impl Clone for Request {
    fn clone(&self) -> Self {
        Self {
            method: self.method.clone(),
            uri: self.uri.clone(),
            version: self.version,
            headers: self.headers.clone(),
            body: self.body.clone(),
            path_params: self.path_params.clone(),
        }
    }
}