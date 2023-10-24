use crate::{
    middleware::auth::sign_jwt,
    util::{
        email::{self, EmailStatus},
        errors::{ApiError, JsonIncoming},
    },
};
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use bcrypt::hash;
use bitfield_struct::bitfield;
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
    email::check_db_for_email(&payload.email, &database).await?;

    let email_code = format!(
        "{:0>6}",
        rand::thread_rng().gen_range(0..999999).to_string()
    );

    email::add_db_email_verification(&payload.email, &email_code, &database).await?;
    email::send_verification_email(&payload.email, &email_code, &payload.name).await?;

    Ok(())
}

pub async fn register_account(
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<RegisterOptions>,
) -> Result<Json<RegisterResult>, (StatusCode, Json<ApiError>)> {
    let email_check = email::check_db_for_email(&payload.email, &database).await?;
    if email_check.status == EmailStatus::AlreadyInUse {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiError {
                error: true,
                message: "Invalid email".to_string(),
            }),
        ));
    }

    email::verify_email(&payload.email, &payload.otp, &database).await?;

    let hash_fn = hash(&payload.password, 13);
    let hashed_password = match hash_fn {
        Ok(pass) => pass,
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

    let uuid = Uuid::new_v4().to_string();
    let flags = UserFlags::new()
        .with_email_verified(true)
        .with_developer(false);

    let raw_flags: u8 = flags.into();
    let token = sign_jwt(&uuid);

	println!("{}", raw_flags);

    let db_req = sqlx::query!(
        "INSERT INTO users (uuid, email, name, auth_method, password, flags) VALUES (?, ?, ?, 0, ?, ?)",
        uuid,
		payload.email,
		payload.name,
        hashed_password,
		raw_flags
    )
    .execute(&database)
    .await;

    match db_req {
        Ok(_) => Ok(Json(RegisterResult { uuid, token })),
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

pub fn router() -> Router {
    return Router::new()
        .route("/exists", post(check_email))
        .route("/verify", post(send_email_verification))
        .route("/new", post(register_account));
}

#[bitfield(u8)]
pub struct UserFlags {
    email_verified: bool,
    developer: bool,
    #[bits(6)]
    _reserved: usize,
}

#[derive(Deserialize)]
pub struct CheckEmailOptions {
    email: String,
}

#[derive(Deserialize)]
pub struct SendEmailVerificationOptions {
    email: String,
    name: String,
}

#[derive(Deserialize)]
pub struct RegisterOptions {
    email: String,
    name: String,
    password: String,
    otp: String,
}

#[derive(Serialize)]
pub struct RegisterResult {
    uuid: String,
    token: String,
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
