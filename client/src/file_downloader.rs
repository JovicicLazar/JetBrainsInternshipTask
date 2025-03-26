use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs::File;

use crate::http_request_builder::HttpRequestBuilder;

// FileDownloader class that retrieves data in chunks 
pub struct FileDownloader {
    base_request: HttpRequestBuilder,
    total_size: Option<usize>,
    data: Vec<u8>,
}

impl FileDownloader {
    pub fn new(base_request: HttpRequestBuilder) -> Self {
        let file_downloader = FileDownloader {
            base_request,
            total_size: None,
            data: Vec::new(),
        };

        file_downloader
    }

    fn get_data(&self, request: &str) -> Result<Vec<u8>, String> {
        let mut stream: TcpStream = TcpStream::connect(self.base_request.get_host())
        .map_err(|e| format!("Failed to connect: {}", e))?;

        stream.write_all(request.as_bytes())
        .map_err(|e| format!("Failed to send request: {}", e))?;
        
        let mut response = Vec::new();
        stream.read_to_end(&mut response)
        .map_err(|e| format!("Failed to read response: {}", e))?;
        
        Ok(response)
    }

    fn initial_fetch(&mut self) -> Result<usize, String> {
        let request = self.base_request.clone().build();
        let response = self.get_data(&request)?;

        let (headers, body) = Self::parse_response(&response)?;
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

        self.data.extend_from_slice(body);
        self.total_size = Some(size);
        
        Ok(size)
    }

    fn fetch_chunk(&mut self, start: usize) -> Result<(), String> {
        let end = self.total_size.unwrap();
        let request = self.base_request.clone()
        .add_header("Range", &format!("bytes={}-{}", start, end))
        .build();
    
        let response = match self.get_data(&request) {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Failed to get data: {}", e)),
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
    
        self.data.extend_from_slice(body);
        Ok(())
    }

    pub fn download_to_file(&mut self, filename: &str) -> Result<(), String> {
        if let Err(e) = self.initial_fetch() {
            return Err(format!("Initial fetch failed: {}", e));
        }

        let mut start = self.data.len();
        
        // case if the whole data is recieved in initial fetch
        // this will not happen with this server
        //if start >= self.total_size.unwrap() {
        //    let mut file = File::create(filename)
        //    .map_err(|e| format!("Failed to create file '{}': {}", filename, e))?;
        //    file.write_all(&self.data)
        //    .map_err(|e| format!("Failed to write to file '{}': {}", filename, e))?;
        //    return Ok(());
        //}

        while start < self.total_size.unwrap() {
            self.fetch_chunk(start)?;
            start = self.data.len();
        }

        let mut file = File::create(filename)
        .map_err(|e| format!("Failed to create file '{}': {}", filename, e))?;
        file.write_all(&self.data)
        .map_err(|e| format!("Failed to write to file '{}': {}", filename, e))?;

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