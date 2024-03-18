use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[allow(dead_code)]
pub enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

#[allow(dead_code)]
pub enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::Unauthorised => (StatusCode::UNAUTHORIZED).into_response(),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}
