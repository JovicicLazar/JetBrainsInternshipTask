use std::fmt::Write;
use std::collections::HashMap;
use crate::http_types::{HttpMethod, HttpVersion};

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

        Self {
            method,
            path: path.to_string(),
            host: host.to_string(),
            port,
            version: HttpVersion::Http1_1,
            headers: HashMap::new(),
        }
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
}

// Tests for HttpRequestBuilder
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_http1_1() {
        let builder = HttpRequestBuilder::new(HttpMethod::Get, "/", "example.com", 80)
        .add_header("User-Agent", "test")
        .version(HttpVersion::Http1_0);

        let request = builder.build();

        let expected = "GET / HTTP/1.0\r\nHost: example.com:80\r\nUser-Agent: test\r\n\r\n";

        assert_eq!(request, expected);
    }

    #[test]
    fn test_post_http2() {
        let builder = HttpRequestBuilder::new(HttpMethod::Post, "/test", "example.com", 8080)
        .version(HttpVersion::Http2)
        .add_header("Content-Type", "application/JSON")
        .add_header("Content-Type", "application/json");

        let request = builder.build();

        let expected = "POST /test HTTP/2\r\nHost: example.com:8080\r\nContent-Type: application/json\r\n\r\n";

        assert_eq!(request, expected);
    }

    #[test]
    fn test_put_http1_0() {
        let request = HttpRequestBuilder::new(HttpMethod::Put, "/resource", "example.com", 80)
        .version(HttpVersion::Http1_0)
        .build();

        let expected = "PUT /resource HTTP/1.0\r\nHost: example.com:80\r\n\r\n";

        assert_eq!(request, expected);
    }

    #[test]
    #[should_panic(expected = "Path must start with '/'")]
    fn test_invalid_path() {
        let _builder = HttpRequestBuilder::new(HttpMethod::Delete, "no_slash", "example.com", 80);
    }

    #[test]
    #[should_panic(expected = "Host cannot be empty")]
    fn test_empty_host() {
        let _builder = HttpRequestBuilder::new(HttpMethod::Delete, "/", "", 80);
    }
}