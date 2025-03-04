mod http_request_builder;
mod file_downloader;
mod http_types;

use std::io::Read;
use std::fs::File;
use http_request_builder::HttpRequestBuilder;
use file_downloader::FileDownloader;
use http_types::{HttpMethod, HttpVersion};

fn main() {
    let base_request = HttpRequestBuilder::new(HttpMethod::Get, "/", "127.0.0.1", 8080)
    .version(HttpVersion::Http1_0)
    .add_header("Connection", "close");
    
    let mut downloader = FileDownloader::new(base_request, 50_000, 10, 1000)
    .expect("Failed to initialize downloader");

    let filename = "downloaded_data.bin";
    match downloader.download_to_file(filename) {
        Ok(()) => {
            println!("Download completed successfully");
            let mut file = File::open(filename).expect("Failed to open file");
            let mut data = Vec::new();
            file.read_to_end(&mut data).expect("Failed to read file");
            println!("Total file size: {}", data.len());
        }
        Err(e) => eprintln!("Download failed: {}", e),
    }
}