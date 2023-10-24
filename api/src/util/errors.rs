use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub status: u16,
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
            status: 400,
            error: true,
            message: "Invalid request".to_string(),
        };
        (self.status, axum::Json(error_info)).into_response()
    }
}

pub fn catch_internal_error<T>(
    call: Result<T, ApiError>,
) -> Result<T, (StatusCode, Json<ApiError>)> {
    match call {
        Ok(res) => Ok(res),
        Err(err) => {
            let status = match StatusCode::from_u16(err.status) {
                Ok(s) => s,
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiError {
                            status: 500,
                            error: true,
                            message: "Internal error".to_string(),
                        }),
                    ))
                }
            };
            Err((status, Json(err)))
        }
    }
}
