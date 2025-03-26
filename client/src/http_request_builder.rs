use std::fmt::Write;
use std::collections::HashMap;
use crate::http_types::{HttpMethod, HttpVersion};

// HTTP request builder designed to simplify request creation and enhance error resistance
#[derive(Clone)]
pub struct HttpRequestBuilder {
    method: HttpMethod,
    path: String,
    host: String,
    port: u16,
    version: HttpVersion,
    headers: HashMap<String, String>,
}

impl HttpRequestBuilder {
    pub fn new(method: HttpMethod, path: &str, host: &str, port: u16) -> Self {
        assert!(path.starts_with('/'), "Path must start with '/'");
        
        assert!(!host.is_empty(), "Host cannot be empty");

        let http_request_builder = Self {
            method,
            path: path.to_string(),
            host: host.to_string(),
            port,
            version: HttpVersion::Http1_0,
            headers: HashMap::new(),
        };

        http_request_builder
    }

    pub fn version(mut self, version: HttpVersion) -> Self {
        self.version = version;
        self
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> String {
        let mut request = String::new();
        writeln!(&mut request, "{} {} {}\r", self.method.as_str(), self.path, self.version.as_str())
        .expect("Failed to write method/path/version");

        writeln!(&mut request, "Host: {}:{}\r", self.host, self.port)
        .expect("Failed to write host");

        for (key, value) in &self.headers {
            writeln!(&mut request, "{}: {}\r", key, value)
            .expect("Failed to write header");
        }

        write!(&mut request, "\r\n").expect("Failed to write final newline");
        request
    }

    pub fn get_host(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_http_version(&self) -> String {
        format!("{}", self.version.as_str())
    }
}