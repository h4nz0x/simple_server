# Simple Server

This is my first Rust project as I learn the language! It's a basic HTTP server built using the Axum web framework.

## What it does

The server provides three endpoints:

- `/` - Returns a simple welcome message
- `/health` - A health check endpoint that returns "OK" 
- `/greet` - Accepts POST requests with JSON payload containing a name and returns a greeting

## Technical Details

The server is built using:
- [Axum](https://github.com/tokio-rs/axum) - A web application framework that focuses on ergonomics and modularity
- [Tokio](https://tokio.rs/) - An asynchronous runtime for Rust
- [Serde](https://serde.rs/) - For JSON serialization/deserialization

The server runs on `localhost:3000` by default.

## How to run

### Using Cargo

1. Make sure you have Rust installed
2. Clone this repository
3. Run `cargo run` in the project directory
4. The server will start on http://127.0.0.1:3000

### Using Docker

1. Make sure you have Docker installed
2. Clone this repository
3. Build the Docker image:
   ```bash
   docker build -t simple-server .
   ```
4. Run the container:
   ```bash
   docker run -p 3000:3000 simple-server
   ```
5. The server will be available at http://localhost:3000

## Example Usage

To test the greeting endpoint:

```bash
curl -X POST http://127.0.0.1:3000/greet -H "Content-Type: application/json" -d '{"name": "John"}'
```
