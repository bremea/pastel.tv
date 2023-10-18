use crate::util::{
    auth::{self, auth_middleware, VerifyTokenResult},
    errors::{ApiError, JsonIncoming},
};
use axum::{
    extract::Path,
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use tower::ServiceBuilder;
use uuid::Uuid;

pub async fn check_email(
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<CheckEmailOptions>,
) -> Result<Json<CheckMailRes>, (StatusCode, Json<ApiError>)> {
    if !mailchecker::is_valid(&payload.email) {
        let api_error_info = ApiError {
            error: true,
            message: "Invalid email address".to_string(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(api_error_info)));
    }

    let db_req = sqlx::query!("SELECT email FROM users WHERE email = ?", payload.email)
        .fetch_one(&database)
        .await;

    match db_req {
        Ok(_) => {
            let acc_match_res = CheckMailRes {
                account_exists: true,
            };
            return Ok(Json(acc_match_res));
        }
        Err(sqlx::Error::RowNotFound) => {
            let acc_match_res = CheckMailRes {
                account_exists: false,
            };
            return Ok(Json(acc_match_res));
        }
        Err(_) => {
            let api_error_info = ApiError {
                error: true,
                message: "Internal Error".to_string(),
            };
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_error_info)));
        }
    }
}

pub fn router() -> Router {
    return Router::new().route("/exists", post(check_email));
}

#[derive(Deserialize)]
pub struct CheckEmailOptions {
    email: String,
}

#[derive(Serialize)]
pub struct CheckMailRes {
    account_exists: bool,
}

#[derive(Serialize)]
pub struct LinkData {
    url: String,
    id: String,
    token: String,
    expires: u64,
    max_usage: u32,
    max_size: u32,
}

#[derive(Serialize)]
pub struct LinkDataDetailed {
    url: String,
    id: String,
    owner: String,
    expires: u64,
    max_usage: u32,
    max_size: u32,
}

#[derive(Serialize)]
pub struct UserLinks {
    links: Vec<LinkData>,
}

#[derive(Debug)]
pub struct RawLinkData {
    pub id: String,
    pub owner: String,
    pub expires: u64,
    pub max_usage: u32,
    pub max_size: u32,
    pub created_at: sqlx::types::time::OffsetDateTime,
}
