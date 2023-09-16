use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login - {payload:?}", "HANDLER");

    // TODO: implement real db connection and login logic

    if payload.username != "olsi" || payload.password != "olsi" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    let body = Json(json!({
        "result": {"success": true},
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
