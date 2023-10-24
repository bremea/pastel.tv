use axum::{http::StatusCode, Json};
use serde::Serialize;
use sqlx::{MySql, Pool};
use std::env;

use super::{consts::SENDER_EMAIL, errors::ApiError};

pub async fn check_db_for_email(
    email: &str,
    database: &Pool<MySql>,
) -> Result<EmailCheckResult, (StatusCode, Json<ApiError>)> {
    if !mailchecker::is_valid(email) {
        let api_error_info = ApiError {
            error: true,
            message: "Invalid email address".to_string(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(api_error_info)));
    }

    let db_req = sqlx::query!("SELECT auth_method FROM users WHERE email = ?", email)
        .fetch_one(database)
        .await;

    match db_req {
        Ok(res) => {
            return Ok(EmailCheckResult {
                status: EmailStatus::AlreadyInUse,
                login_method: res.auth_method,
            });
        }
        Err(sqlx::Error::RowNotFound) => {
            return Ok(EmailCheckResult {
                status: EmailStatus::Available,
                login_method: 0,
            });
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

pub async fn add_db_email_verification(
    email: &str,
    code: &str,
    database: &Pool<MySql>,
) -> Result<(), (StatusCode, Json<ApiError>)> {
    let db_req = sqlx::query!("INSERT INTO email_verification (email, code, expires) VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 1 HOUR))", email, code)
        .execute(database)
        .await;

    match db_req {
        Ok(_) => Ok(()),
        Err(_) => {
            let api_error_info = ApiError {
                error: true,
                message: "Internal Error".to_string(),
            };
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_error_info)));
        }
    }
}

pub async fn verify_email(
    email: &str,
    code: &str,
    database: &Pool<MySql>,
) -> Result<(), (StatusCode, Json<ApiError>)> {
    let db_req = sqlx::query!(
        "SELECT email FROM email_verification WHERE email = ? AND code = ? AND expires > NOW()",
        email,
        code
    )
    .fetch_one(database)
    .await;

    match db_req {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => {
            let api_error_info = ApiError {
                error: true,
                message: "Invalid code".to_string(),
            };
            return Err((StatusCode::BAD_REQUEST, Json(api_error_info)));
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

pub async fn send_verification_email(
    email: &str,
    code: &str,
    name: &str,
) -> Result<(), (StatusCode, Json<ApiError>)> {
    let email_body = format!(
        "Hey {name}! Your verification code for pastel.tv is {code}",
        name = name,
        code = code
    );
    let email_message = ResendEmailData {
        from: SENDER_EMAIL.to_string(),
        to: email.to_string(),
        subject: "Verify your pastel.tv account".to_string(),
        text: email_body,
    };

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.resend.com/emails")
        .bearer_auth(&env::var("RESEND_KEY").expect("Missing RESEND_KEY"))
        .json(&email_message)
        .send()
        .await;

    match resp {
        Ok(_) => Ok(()),
        Err(_) => {
            let api_error_info = ApiError {
                error: true,
                message: "Internal Error".to_string(),
            };
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_error_info)));
        }
    }
}

pub struct EmailCheckResult {
    pub status: EmailStatus,
    pub login_method: u8,
}

#[derive(Serialize)]
pub struct ResendEmailData {
    from: String,
    to: String,
    subject: String,
    text: String,
}

#[derive(PartialEq)]
pub enum EmailStatus {
    AlreadyInUse,
    Available,
}
