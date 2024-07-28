use std::collections::HashMap;
use std::fmt;
use std::slice::{Iter, IterMut};

/// Collection represents a collection of Routes and/or nested Collections with Environments.
#[derive(Debug, Clone)]
pub struct Collection {
    requests: Vec<Request>,
    name: String,
}

impl Collection {
    pub fn new(name: String, requests: Vec<Request>) -> Self {
        Self { requests, name }
    }

    pub fn add_request(&mut self, route: Request) {
        self.requests.push(route);
    }

    pub fn is_empty(&self) -> bool {
        return self.requests.is_empty();
    }

    pub fn name(&self) -> String {
        self.name.clone()
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

impl Default for Collection {
    fn default() -> Self {
        Collection {
            name: String::from("Untitled Collection"),
            requests: vec![],
        }
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
#[derive(Debug, Default, Clone, Copy)]
pub enum HttpMethod {
    #[default]
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Option,
}

impl HttpMethod {
    pub fn next(&self) -> HttpMethod {
        match self {
            HttpMethod::Get => HttpMethod::Post,
            HttpMethod::Post => HttpMethod::Put,
            HttpMethod::Put => HttpMethod::Patch,
            HttpMethod::Patch => HttpMethod::Delete,
            HttpMethod::Delete => HttpMethod::Option,
            HttpMethod::Option => HttpMethod::Get,
        }
    }

    pub fn prev(&self) -> HttpMethod {
        match self {
            HttpMethod::Get => HttpMethod::Option,
            HttpMethod::Post => HttpMethod::Get,
            HttpMethod::Put => HttpMethod::Post,
            HttpMethod::Patch => HttpMethod::Put,
            HttpMethod::Delete => HttpMethod::Patch,
            HttpMethod::Option => HttpMethod::Delete,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Option => "OPTION",
        }
    }
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
