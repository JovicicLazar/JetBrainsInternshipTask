mod http_request_builder;
mod file_downloader;
mod http_types;

use std::io::Read;
use std::fs::File;
use http_request_builder::HttpRequestBuilder;
use file_downloader::FileDownloader;
use http_types::{HttpMethod, HttpVersion};
use sha2::{Sha256, Digest};

const HOST: &str     = "127.0.0.1";
const PATH: &str     = "/";
const PORT: u16      = 8080;
const FILENAME: &str = "downloaded_data.bin";

fn main() {
    let base_request = HttpRequestBuilder::new(HttpMethod::Get, &PATH, &HOST, PORT)
    .version(HttpVersion::Http1_0)
    .add_header("Connection", "close");

    let mut downloader = FileDownloader::new(base_request);

    print!("Downloading data...\n");
    
    if let Err(e) = downloader.download_to_file(&FILENAME) {
        eprintln!("Download failed: {}", e);
        return;
    }

    println!("Download complete!");

    if let Err(e) = print_hash(&FILENAME) {
        eprintln!("Failed to calculate hash: {}", e);
        return;
    }
}

fn print_hash(file_name: &str) -> Result<(), String> {
    let mut file = File::open(file_name)
    .map_err(|e| format!("Failed to open downloaded file: {}", e))?;

    let mut data = Vec::new();

    file.read_to_end(&mut data)
    .map_err(|e| format!("Failed to read downloaded file: {}", e))?;

    println!("Total file size: {} bytes", data.len());

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();

    println!("Result of sha256 crate: {:x}", result);

    Ok(())
}