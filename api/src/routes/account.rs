use crate::util::{
    email::{self, EmailStatus},
    errors::{ApiError, JsonIncoming},
};
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use uuid::Uuid;

pub async fn check_email(
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<CheckEmailOptions>,
) -> Result<Json<CheckMailRes>, (StatusCode, Json<ApiError>)> {
    let email_check = email::check_db_for_email(&payload.email, &database).await;
    match email_check {
        Ok(res) => match res.status {
            EmailStatus::AlreadyInUse => Ok(Json(CheckMailRes {
                login_method: res.login_method,
                new_account: false,
            })),
            EmailStatus::Available => Ok(Json(CheckMailRes {
                login_method: 0,
                new_account: true,
            })),
        },
        Err(res) => {
            return Err(res);
        }
    }
}

pub async fn send_email_verification(
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<SendEmailVerificationOptions>,
) -> Result<(), (StatusCode, Json<ApiError>)> {
    let email_check = email::check_db_for_email(&payload.email, &database).await;
    let _ = match email_check {
        Ok(res) => res,
        Err(res) => {
            return Err(res);
        }
    };

    let email_code = format!("{:0>6}", rand::thread_rng().gen_range(0..999999).to_string());
	email::add_db_email_verification(&payload.email, &email_code, &database).await?;
	//email::send_verification_email(&payload.email, &email_code, &payload.name).await?;

	Ok(())
}

pub fn router() -> Router {
    return Router::new()
        .route("/exists", post(check_email))
        .route("/verify", post(send_email_verification));
}

#[derive(Deserialize)]
pub struct CheckEmailOptions {
    email: String,
}

#[derive(Deserialize)]
pub struct SendEmailVerificationOptions {
    email: String,
    name: String
}

#[derive(Serialize)]
pub struct CheckMailRes {
    login_method: u8,
    new_account: bool,
}

#[derive(Serialize)]
pub struct EmailVerificationResult {
    error: bool,
}
