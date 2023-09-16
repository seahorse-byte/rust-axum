#![allow(unused)]

pub use self::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    // START SERVER
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // STATIC ROUTES
    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }

    // HELLO ROUTES
    fn routes_hello() -> Router {
        Router::new()
            .route("/hello", get(handler_hello))
            .route("/hello2/:name", get(handler_hello2))
    }

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
        let name = params.name.as_deref().unwrap_or("World");
        Html(format!("<h1>Helooo {name}</h1>"))
    }

    async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
        Html(format!("<h1>Helooo {name}</h1>"))
    }
}
