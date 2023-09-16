#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(handler_hello));

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
        let name = params.name.as_deref().unwrap_or("World");
        Html(format!("<h1>Helooo {name}</h1>"))
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
