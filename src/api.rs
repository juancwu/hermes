use ratatui::style;
use std::collections::HashMap;
use std::fmt::{self};
use std::slice::Iter;

/// Collection represents a collection of Routes and/or nested Collections with Environments.
#[derive(Debug, Clone)]
pub struct Collection {
    identifier: String,
    name: String,
    requests: Vec<Request>,
    enable_environment: bool,
    active_environment: String,
    environments: HashMap<String, HashMap<String, String>>,
}

impl Collection {
    pub fn add_request(&mut self, route: Request) {
        self.requests.push(route);
    }

    pub fn get_request_count(&self) -> usize {
        self.requests.len()
    }

    pub fn is_empty(&self) -> bool {
        return self.requests.is_empty();
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = identifier;
    }

    pub fn identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn new_environment(&mut self, environment_name: String) {
        self.environments.insert(environment_name, HashMap::new());
    }

    pub fn set_active_environment(&mut self, enviroment_name: String) {
        self.active_environment = enviroment_name;
    }

    pub fn add_environment_entry(&mut self, key: String, value: String) {
        if let Some(env) = self.environments.get_mut(&self.active_environment) {
            env.insert(key, value);
        }
    }

    pub fn get_active_environment(&mut self) -> Option<&mut HashMap<String, String>> {
        self.environments.get_mut(&self.active_environment)
    }

    pub fn enable_active_environment(&mut self) {
        self.enable_environment = true;
    }

    pub fn disable_active_environment(&mut self) {
        self.enable_environment = false;
    }

    pub fn iter(&self) -> Iter<'_, Request> {
        self.requests.iter()
    }

    // Import std::slice::IterMut
    // pub fn iter_mut(&mut self) -> IterMut<'_, Request> {
    //     self.requests.iter_mut()
    // }
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
            identifier: String::new(),
            enable_environment: false,
            active_environment: String::new(),
            environments: HashMap::new(),
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

    /// Gets a clone of the name of the request.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Gets the http method of the request.
    pub fn get_method(&self) -> HttpMethod {
        self.method
    }

    /// Gets a clone of the url of the request.
    pub fn get_url(&self) -> String {
        self.url.clone()
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

    pub fn color(&self) -> style::Color {
        match self {
            HttpMethod::Get => style::Color::Green,
            HttpMethod::Post => style::Color::Yellow,
            HttpMethod::Put => style::Color::Blue,
            HttpMethod::Patch => style::Color::LightBlue,
            HttpMethod::Delete => style::Color::Red,
            HttpMethod::Option => style::Color::LightCyan,
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
