mod http_request_builder;
mod file_downloader;
mod progress_bar;
mod http_types;
mod config;

use std::io::Read;
use std::fs::File;
use std::process::Command;
use http_request_builder::HttpRequestBuilder;
use file_downloader::FileDownloader;
use http_types::{HttpMethod, HttpVersion};
use config::Config;

const INI_FILE_PATH: &str = "../../src/client_setup.ini"; // this path, because when you build the code, you will run it from target/debug
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PATH: &str = "/";
const DEFAULT_PORT: i32 = 8080;
const DEFAULT_CHUNK_SIZE: i32 = 50_000;
const DEFAULT_RETRIES: i32 = 10;
const DEFAULT_TIMEOUT: i32 = 1000;
const DOWNLOAD_FILENAME: &str = "downloaded_data.bin";

fn main() {
    let mut config_file = Config::new();
    if !config_file.load_file(INI_FILE_PATH) {
        println!("Failed to load {}, exiting.", INI_FILE_PATH);
        return;
    }

    let host = match config_file.get_as_string("request", "host") {
        Some(host) => host,
        None => DEFAULT_HOST.to_string(),
    };

    let path = match config_file.get_as_string("request", "path") {
        Some(path) => path,
        None => DEFAULT_PATH.to_string(),
    };

    let port = match config_file.get_as_int("request", "port") {
        Some(port) if port >= 0 && port <= 65535 => port as u16,
        _ => DEFAULT_PORT as u16,
    };

    let chunk_size = match config_file.get_as_int("downloader", "chunk_size") {
        Some(size) if size > 0 => size as usize,
        _ => DEFAULT_CHUNK_SIZE as usize,
    };

    let retries = match config_file.get_as_int("downloader", "retries") {
        Some(retries) if retries >= 0 => retries as usize,
        _ => DEFAULT_RETRIES as usize,
    };

    let timeout = match config_file.get_as_int("downloader", "timeout") {
        Some(timeout) if timeout >= 0 => timeout as u64,
        _ => DEFAULT_TIMEOUT as u64,
    };

    let base_request = HttpRequestBuilder::new(HttpMethod::Get, &path, &host, port)
    .version(HttpVersion::Http1_0)
    .add_header("Connection", "close");

    let mut downloader = match FileDownloader::new(base_request, chunk_size, retries, timeout) {
        Ok(downloader) => downloader,
        Err(e) => {
            eprintln!("Failed to initialize downloader: {}", e);
            return;
        }
    };

    match downloader.download_to_file(DOWNLOAD_FILENAME) {
        Ok(()) => {
            println!("Download completed successfully");
            let mut file = match File::open(DOWNLOAD_FILENAME) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open downloaded file: {}", e);
                    return;
                }
            };
            let mut data = Vec::new();
            if let Err(e) = file.read_to_end(&mut data) {
                eprintln!("Failed to read downloaded file: {}", e);
                return;
            }
            println!("Total file size: {} bytes", data.len());

            let hash = calculate_sha256(DOWNLOAD_FILENAME);
            if !hash.is_empty() {
                println!("SHA-256 hash: {}", hash);
            } else {
                println!("Failed to compute SHA-256 hash.");
            }
        }
        Err(e) => eprintln!("Download failed: {}", e),
    }
}

// Function that uses Linux’s `sha256sum` command to get the SHA-256 hash of a downloaded file.
// Just a heads-up: I tossed this in for fun, so it’s pretty basic and not built to last.
// It’s Linux-only and if something goes wrong, it just hands back an empty string.
fn calculate_sha256(filename: &str) -> String {
    let output = match Command::new("sha256sum").arg(filename).output() {
        Ok(output) => output,
        Err(_) => return String::new(),
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.split_whitespace().next().unwrap_or("").to_string()
    } else {
        String::new()
    }
}