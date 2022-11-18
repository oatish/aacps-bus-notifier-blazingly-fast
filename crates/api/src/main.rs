use axum::{
    routing::{get},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello));

    let addr = SocketAddr::from(([127, 0 , 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, Shane!"
}