use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs::OpenOptions;
use std::time::Duration;

use crate::http_request_builder::HttpRequestBuilder;
use crate::progress_bar::ProgressBar;

const PROGRESS_BAR_WIDTH: usize = 50;
const PROGRESS_BAR_TOTAL: usize = 0;

pub struct FileDownloader {
    base_request: HttpRequestBuilder,
    progress_bar: ProgressBar,
    total_size: Option<usize>,
    chunk_size: usize,
    retries: usize,
    timeout: u64,
}

impl FileDownloader {
    pub fn new(base_request: HttpRequestBuilder, chunk_size: usize, retries: usize, timeout: u64) -> Result<Self, String> {
        let file_downloader = FileDownloader {
            base_request,
            progress_bar: ProgressBar::new(PROGRESS_BAR_TOTAL, PROGRESS_BAR_WIDTH),
            total_size: None,
            chunk_size,
            retries,
            timeout,
        };

        Ok(file_downloader)
    }

    fn get_data(&self, request: &str) -> Result<Vec<u8>, String> {
        let mut stream = TcpStream::connect(self.base_request.get_host())
        .map_err(|e| format!("Failed to connect: {}", e))?;
        
        stream.set_read_timeout(Some(Duration::from_millis(self.timeout)))
        .map_err(|e| format!("Failed to set read timeout: {}", e))?;
        stream.set_write_timeout(Some(Duration::from_millis(self.timeout)))
        .map_err(|e| format!("Failed to set write timeout: {}", e))?;

        stream.write_all(request.as_bytes())
        .map_err(|e| format!("Failed to send request: {}", e))?;
        
        let mut response = Vec::new();
        stream.read_to_end(&mut response)
        .map_err(|e| format!("Failed to read response: {}", e))?;
        
        Ok(response)
    }

    fn fetch_total_size(&mut self) -> Result<usize, String> {
        let request = self.base_request.clone().build();
        let response = self.get_data(&request)?;

        let (headers, _) = Self::parse_response(&response)?;
        let expected_status = format!("{} 200", self.base_request.get_http_version());
        if !headers.starts_with(&expected_status) {
            return Err(format!(
                "Expected '{}', got '{}'",
                &expected_status,
                headers.lines().next().unwrap_or("no status")
            ));
        }

        let size = headers.lines()
        .find(|line| line.starts_with("Content-Length:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|val| val.trim().parse::<usize>().ok())
        .ok_or("Missing or invalid Content-Length")?;

        self.total_size = Some(size);
        
        Ok(size)
    }

    fn fetch_chunk(&self, request: &str, expected_size: usize) -> Result<Vec<u8>, String> {
        let mut attempt = 1;
        while attempt <= self.retries {
            self.progress_bar.print();
            println!("(attempt {})", attempt);

            let response = match self.get_data(request) {
                Ok(resp) => resp,
                Err(e) => {
                    if attempt == self.retries {
                        return Err(format!("Failed to get data after {} retries: {}", self.retries, e));
                    }
                    std::thread::sleep(Duration::from_millis(self.timeout));
                    attempt += 1;
                    continue;
                }
            };

            let (headers, body) = Self::parse_response(&response)?;
            let expected_status = format!("{} 206", self.base_request.get_http_version());
            if !headers.starts_with(&expected_status) {
                return Err(format!(
                    "Expected '{} 206', got '{}'",
                    self.base_request.get_http_version(),
                    headers.lines().next().unwrap_or("no status")
                ));
            }

            let received_size = body.len();
            if received_size < expected_size {
                if attempt == self.retries {
                    return Err(format!("Incomplete chunk after {} retries", self.retries));
                }
                std::thread::sleep(Duration::from_millis(100));
                attempt += 1;
                continue;
            }

            return Ok(body.to_vec());
        }
        Err(format!("Failed to fetch chunk after {} retries", self.retries + 1))
    }

    pub fn download_to_file(&mut self, filename: &str) -> Result<(), String> {
        let total_size = match self.total_size {
            Some(size) => size,
            None => self.fetch_total_size()?,
        };

        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .map_err(|e| format!("Failed to open file: {}", e))?;

        let mut start = 0;
        self.progress_bar.set_total(total_size);

        self.progress_bar.update(0);

        while start < total_size {
            let end = (start + self.chunk_size).min(total_size);
            let expected_size = end - start;

            let request = self.base_request.clone()
            .add_header("Range", &format!("bytes={}-{}", start, end))
            .build();

            let body = self.fetch_chunk(&request, expected_size)?;
            file.write_all(&body)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
            
            start = end;

            self.progress_bar.update(start);
        }

        self.progress_bar.finish();

        Ok(())
    }

    fn parse_response(response: &[u8]) -> Result<(&str, &[u8]), String> {
        let split_pos = response.windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or("No header-body separator found")?;

        let headers = std::str::from_utf8(&response[..split_pos])
        .map_err(|e| format!("Invalid UTF-8 in headers: {}", e))?;

        let body = &response[split_pos + 4..];

        Ok((headers, body))
    }
}