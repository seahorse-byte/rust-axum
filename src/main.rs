#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(handler_hello));

    async fn handler_hello() -> impl IntoResponse {
        // println!("->> {:<12} - handler_hello", "HANDLER");
        Html("<h1>Hello, World!</h1>")
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
