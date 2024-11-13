use axum::{
    routing::{get, post},
    http::StatusCode,
    Router, Json,
};
use hyper::Server;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(root))                // Root endpoint
        .route("/health", get(health_check))  // Health check endpoint
        .route("/greet", post(greet));        // Greeting endpoint with JSON payload

    // Define the address to listen on (binding to all interfaces)
    let addr = "0.0.0.0:8000".parse::<SocketAddr>().unwrap();
    println!("Listening on http://{}", addr);

    // Run the server using hyper
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for the root route
async fn root() -> &'static str {
    "Simple Server 1.0"
}

// Health check handler
async fn health_check() -> &'static str {
    "OK"
}

// Struct for the JSON request payload in the greet route
#[derive(Deserialize)]
struct GreetRequest {
    name: String,
}

// Struct for the JSON response payload in the greet route
#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

// Handler for the greet route
async fn greet(Json(payload): Json<GreetRequest>) -> (StatusCode, Json<GreetResponse>) {
    let response = GreetResponse {
        message: format!("Hello, {}!", payload.name),
    };
    (StatusCode::OK, Json(response))
}
