# JetBrains Internship Task

This repository contains a Rust-based file downloader client and a Python-based buggy server. The client downloads a file from the server using HTTP range requests, with configurable settings loaded from an INI file (`client_setup.ini`).

## Project Structure
- **`client/`**: Rust client code (the file downloader).
- **`server/`**: Python server code (`buggy_server.py`).
- **`client_setup.ini`**: Configuration file for the client (place in `client/config`).

## Prerequisites
- **Rust**: Install via [rustup](https://rustup.rs/) (`rustc` and `cargo` required).
- **Python 3**: Ensure `python3` is installed (for the server).

## How to Build

Follow these steps from the root directory (`JetBrainsInternshipTask`)

1. Navigate to the client directory:
   ```bash
   cd client
   ```

2. Build the project:
   ```bash
   cargo build
   ```
   - This compiles the Rust code into `target/debug/`.
   - The executable will be `target/debug/client`.
   - From the root directory `client/target/debug/client`.

## How to Run

To run the program, start the server first, then execute the client. All commands assume you’re starting from `JetBrainsInternshipTask`.

### Start the Server

1. Navigate to the server directory:
   ```bash
   cd server
   ```

2. Run the Python server:
   ```bash
   python3 buggy_server.py
   ```
   - Keep this terminal open.

### Run the Client

1. Open a new terminal and navigate to the client’s debug directory:
   ```bash
   cd client/target/debug
   ```

2. Run the client:
   ```bash
   ./client
   ```
   - Loads settings from `client_setup.ini`, downloads `downloaded_data.bin`, and computes its SHA-256 hash.

#### Example `client_setup.ini`
```ini
[request]
host=127.0.0.1
port=8080
path=/

[downloader]
chunk_size=50000
retries=10
timeout=1000
```
The `client_setup.ini` file is located in `/client/src` directory.

## Expected output
   ```bash
   [==================================================] 100.0% (874536/874536 bytes)
   Download completed successfully
   Total file size: 874536 bytes
   SHA-256 hash: c98668524fb5400df1996ec61e9c5a2fabdddf57dcd839f6f1834026a2aa8c7d
   ```

