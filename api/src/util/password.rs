use axum::Json;
use bcrypt::{hash, verify};
use reqwest::StatusCode;
use sqlx::{MySql, Pool};

use super::errors::ApiError;

pub async fn hash_password(password: &str) -> Result<String, (StatusCode, Json<ApiError>)> {
    let hashed = hash(password, 13);
    match hashed {
        Ok(hashed_password) => Ok(hashed_password),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: true,
                message: "Internal Error".to_string(),
            }),
        )),
    }
}

pub async fn verify_password(
    email: &str,
    password: &str,
    database: &Pool<MySql>,
) -> Result<bool, (StatusCode, Json<ApiError>)> {
    let db_req = sqlx::query!("SELECT password FROM users WHERE email = ?", email)
        .fetch_one(database)
        .await;
    let hashed_password = match db_req {
        Ok(hashed) => match hashed.password {
            Some(pass) => pass,
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ApiError {
                        error: true,
                        message: "Wrong login method".to_string(),
                    }),
                ));
            }
        },
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    error: true,
                    message: "Internal Error".to_string(),
                }),
            ));
        }
    };

    match verify(password, &hashed_password) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
