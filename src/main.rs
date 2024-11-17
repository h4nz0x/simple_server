use axum::{
    routing::{get, post},
    http::StatusCode,
    Router, Json,
};
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use hyper::Server;

#[tokio::main]
async fn main() {
    // Set up the tracing subscriber for logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Define routes
    let app = Router::new()
        .route("/", get(root))                // Root endpoint
        .route("/health", get(health_check))  // Health check endpoint
        .route("/greet", post(greet))        // Greeting endpoint with JSON payload
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true)) // log request headers
                .on_response(DefaultOnResponse::new().include_headers(true)) // log response headers
        );

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000)); // Bind to all Localhost
    info!("Listening on http://{}", addr);

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
