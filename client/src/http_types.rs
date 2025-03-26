// Types for HttpRequestBuilder.
#[derive(Clone)]
pub enum HttpMethod {
    Get,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
        }
    }
}

#[derive(Clone)]
pub enum HttpVersion {
    Http1_0,
}

impl HttpVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpVersion::Http1_0 => "HTTP/1.0",
        }
    }
}