use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub error: bool,
    pub message: String,
}

#[derive(Debug)]
pub struct ApiErrorJsonHandler {
    pub status: StatusCode,
    pub message: String,
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiErrorJsonHandler))]
pub struct JsonIncoming<T>(pub T);

impl From<JsonRejection> for ApiErrorJsonHandler {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl IntoResponse for ApiErrorJsonHandler {
    fn into_response(self) -> axum::response::Response {
        let error_info = ApiError {
            error: true,
            message: self.message,
        };
        (self.status, axum::Json(error_info)).into_response()
    }
}
