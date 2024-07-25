use std::collections::HashMap;
use std::fmt;
use std::slice::{Iter, IterMut};

/// Collection represents a collection of Routes and/or nested Collections with Environments.
#[derive(Debug, Clone, Default)]
pub struct Collection {
    requests: Vec<Request>,
}

impl Collection {
    pub fn new(requests: Vec<Request>) -> Self {
        Self { requests }
    }

    pub fn add_request(&mut self, route: Request) {
        self.requests.push(route);
    }

    pub fn is_empty(&self) -> bool {
        return self.requests.is_empty();
    }

    pub fn iter(&self) -> Iter<'_, Request> {
        self.requests.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Request> {
        self.requests.iter_mut()
    }
}

impl IntoIterator for Collection {
    type Item = Request;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.requests.into_iter()
    }
}

/// Request represents a single route that is store in a Collection.
/// It stores the method, url, headers, and body the Request would use.
#[derive(Debug, Clone)]
pub struct Request {
    name: String,
    method: HttpMethod,
    url: String,
    body: Option<String>,
    body_type: Option<HttpBody>,
    /// a list of key-value pairs for the headers.
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new(
        name: String,
        method: HttpMethod,
        url: String,
        body: Option<String>,
        body_type: Option<HttpBody>,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            name,
            method,
            url,
            body,
            body_type,
            headers,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.name, self.method, self.url)
    }
}

/// HttpMethod is the method that a Request should use to call the API.
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Option,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Option => "OPTION",
        };
        write!(f, "{}", method)
    }
}

/// HttpBody is the type of body that is being sent in the Request.
#[derive(Debug, Clone, Copy)]
pub enum HttpBody {
    Json,
    FormUrlEncoded,
}