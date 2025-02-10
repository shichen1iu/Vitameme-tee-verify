// crates/api-server/src/main.rs
mod error;
mod handler;
mod utils;

use crate::handler::twitter::verify_and_sign::*;

const CURRENT_VERSION: &str = "v1";

use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::{routing::post, Router};

#[derive(serde::Serialize)]
struct ApiResponse<T> {
    code: u16,
    message: String,
    data: Option<T>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/verify", post(verify_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn verify_handler(Json(payload): Json<(String, String)>) -> impl IntoResponse {
    println!("Received request with payload: {:?}", payload);
    println!("payload.0: {:?}", payload.0);
    println!("payload.1: {:?}", payload.1);

    if payload.0.is_empty() || payload.1.is_empty() {
        println!("Invalid parameters received");
        return Json(ApiResponse {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: "Invalid parameters".to_string(),
            data: None,
        });
    }

    match verify_and_sign(&payload.0, &payload.1) {
        Ok(response) => {
            println!("Request processed successfully");
            Json(ApiResponse {
                code: StatusCode::OK.as_u16(),
                message: "success".to_string(),
                data: Some(response),
            })
        }
        Err(err) => {
            println!("Error processing request: {}", err);
            Json(ApiResponse {
                code: StatusCode::BAD_REQUEST.as_u16(),
                message: err.to_string(),
                data: None,
            })
        }
    }
}
