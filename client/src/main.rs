mod http_request_builder;
mod http_types;

use http_request_builder::HttpRequestBuilder;
use http_types::{HttpMethod, HttpVersion};

fn main() {
    let request = HttpRequestBuilder::new(HttpMethod::Get, "/", "127.0.0.1", 8080)
    .add_header("Connection", "close")
    .add_header("Range", "bytes=0-100")
    .version(HttpVersion::Http1_1)
    .build();
    
    print!("Request \n{}", request);
}
