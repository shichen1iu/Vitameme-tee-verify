use std::fmt;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    // 数据不存在
    NotFound(String),
    // 签名验证失败
    SignatureError(String),
    // 无效的消息
    InvalidMessage(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found : {}", msg),
            ApiError::SignatureError(msg) => write!(f, "Signature Error: {}", msg),
            ApiError::InvalidMessage(msg) => write!(f, "Invalid Message: {}", msg),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::SignatureError(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::InvalidMessage(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(ErrorResponse {
            code: status.as_u16(),
            message,
        });

        (status, body).into_response()
    }
}
