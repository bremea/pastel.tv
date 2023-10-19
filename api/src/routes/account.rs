use crate::{
    middleware::auth::{self, auth_middleware, VerifyTokenResult},
    util::errors::{ApiError, JsonIncoming},
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

    let db_req = sqlx::query!("SELECT auth_method FROM users WHERE email = ?", payload.email)
        .fetch_one(&database)
        .await;

    match db_req {
        Ok(res) => {
            let acc_match_res = CheckMailRes {
                login_method: res.auth_method,
				new_account: false
            };
            return Ok(Json(acc_match_res));
        }
        Err(sqlx::Error::RowNotFound) => {
            let acc_match_res = CheckMailRes {
                login_method: 0,
				new_account: true
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
    login_method: u8,
	new_account: bool
}
