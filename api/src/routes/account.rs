use std::env;

use crate::{
    database::{email, tokens, user},
    middleware::auth::{auth_middleware, VerifyTokenResult},
    util::{
        errors::{catch_internal_error, ApiError, JsonIncoming},
        password,
    },
};
use axum::{
    http::StatusCode,
    middleware,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use uuid::Uuid;

pub async fn check_email(
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<CheckEmailOptions>,
) -> Result<Json<CheckMailRes>, (StatusCode, Json<ApiError>)> {
    // check for legit email address
    catch_internal_error(email::check_for_valid_email_address(&payload.email))?;

    // query db for email
    let try_get_user =
        catch_internal_error(user::get_user_by_email(&payload.email, &database).await)?;

    // send result depending if email is registered already
    match try_get_user {
        Some(res) => Ok(Json(CheckMailRes {
            login_method: res.auth_method,
            new_account: false,
        })),
        None => Ok(Json(CheckMailRes {
            login_method: 0,
            new_account: true,
        })),
    }
}

pub async fn send_email_verification(
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<SendEmailVerificationOptions>,
) -> Result<(), (StatusCode, Json<ApiError>)> {
    // check for legit email address
    catch_internal_error(email::check_for_valid_email_address(&payload.email))?;

    // generate code
    let email_code = format!(
        "{:0>6}",
        rand::thread_rng().gen_range(0..999999).to_string()
    );

    // add code to db and send
    catch_internal_error(
        email::add_db_email_verification(&payload.email, &email_code, &database).await,
    )?;
    catch_internal_error(
        email::send_verification_email(&payload.email, &email_code, &payload.name).await,
    )?;

    Ok(())
}

pub async fn login(
    cookies: CookieJar,
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<LoginOptions>,
) -> Result<(CookieJar, Json<LoginResult>), (StatusCode, Json<ApiError>)> {
    // get user with email
    let user = match catch_internal_error(user::get_user_by_email(&payload.email, &database).await)?
    {
        Some(some_user) => some_user,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    status: 401,
                    error: true,
                    message: "Invalid email or password".to_string(),
                }),
            ));
        }
    };

    // check user has password set
    let hashed_password = match user.password {
        Some(pass) => pass,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    status: 401,
                    error: true,
                    message: "Invalid email or password".to_string(),
                }),
            ));
        }
    };

    // check for valid password
    if !catch_internal_error(password::verify_password(&payload.password, &hashed_password).await)?
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                status: 401,
                error: true,
                message: "Invalid email or password".to_string(),
            }),
        ));
    }

    // generate access and refresh tokens
    let tokens = catch_internal_error(tokens::generate_tokens(user.uuid, &database).await)?;

    // create refresh token cookie
    let refresh_token_cookie = match env::var_os("DEVELOPMENT_MODE") {
        Some(_) => Cookie::build("refresh_token", tokens.refresh_token)
            .http_only(true)
            .finish(),
        None => Cookie::build("refresh_token", tokens.refresh_token)
            .same_site(SameSite::Strict)
            .secure(true)
            .http_only(true)
            .finish(),
    };

    // set refresh token cookie, and return access token and user id
    Ok((
        cookies.add(refresh_token_cookie),
        Json(LoginResult {
            access_token: tokens.access_token,
            user_id: tokens.user_id,
        }),
    ))
}

pub async fn register_account(
    cookies: CookieJar,
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<RegisterOptions>,
) -> Result<(CookieJar, Json<LoginResult>), (StatusCode, Json<ApiError>)> {
    // make sure email is available
    if catch_internal_error(user::get_user_by_email(&payload.email, &database).await)?.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiError {
                status: 409,
                error: true,
                message: "Invalid email".to_string(),
            }),
        ));
    }

    // check OTP is correct
    catch_internal_error(email::verify_email(&payload.email, &payload.otp, &database).await)?;

    // hash password
    let hashed_password = catch_internal_error(password::hash_password(&payload.password).await)?;

    // generate UUID and user flags
    let uuid = Uuid::new_v4().to_string();
    let flags = user::UserFlags::new()
        .with_email_verified(true)
        .with_developer(false);

    // add user to db
    let new_user = user::User {
        uuid: uuid.clone(),
        email: payload.email,
        name: payload.name,
        auth_method: 0,
        password: Some(hashed_password),
        flags,
    };
    catch_internal_error(user::add_user(&new_user, &database).await)?;

    // generate access and refresh tokens
    let tokens = catch_internal_error(tokens::generate_tokens(new_user.uuid, &database).await)?;

    // create refresh token cookie
    let refresh_token_cookie = Cookie::build("refresh_token", tokens.refresh_token)
        .same_site(SameSite::Strict)
        .secure(true)
        .http_only(true)
        .finish();

    // set refresh token cookie, and return access token and user id
    Ok((
        cookies.add(refresh_token_cookie),
        Json(LoginResult {
            access_token: tokens.access_token,
            user_id: tokens.user_id,
        }),
    ))
}

pub async fn current_user(
    Extension(verification): Extension<VerifyTokenResult>,
    Extension(database): Extension<Pool<MySql>>,
) -> Result<Json<PartialUser>, (StatusCode, Json<ApiError>)> {
    // get current user
    let user =
        match catch_internal_error(user::get_user_by_uuid(&verification.uuid, &database).await)? {
            Some(some_user) => some_user,
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ApiError {
                        status: 400,
                        error: true,
                        message: "No such user".to_string(),
                    }),
                ))
            }
        };

    // convert to partial user result
    let partial_user = PartialUser {
        email: user.email,
        name: user.name,
        uuid: user.uuid,
        flags: user.flags.into(),
    };

    Ok(Json(partial_user))
}

pub async fn get_tokens(
    Extension(database): Extension<Pool<MySql>>,
    cookies: CookieJar,
) -> Result<(CookieJar, Json<LoginResult>), (StatusCode, Json<ApiError>)> {
    let refresh_token = match cookies.get("refresh_token") {
        Some(cookie) => cookie.value(),
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    status: 401,
                    error: true,
                    message: "Missing refresh token".to_string(),
                }),
            ))
        }
    };

    // get new tokens
    let new_tokens =
        catch_internal_error(tokens::login_with_refresh_token(refresh_token, &database).await)?;

    // create refresh token cookie
    let refresh_token_cookie = Cookie::build("refresh_token", new_tokens.refresh_token)
        .same_site(SameSite::Strict)
        .secure(true)
        .http_only(true)
        .finish();

    // set refresh token cookie, and return access token and user id
    Ok((
        cookies.add(refresh_token_cookie),
        Json(LoginResult {
            access_token: new_tokens.access_token,
            user_id: new_tokens.user_id,
        }),
    ))
}

pub fn router() -> Router {
    return Router::new()
        .route("/", get(current_user))
        .layer(middleware::from_fn(auth_middleware))
        .route("/token", get(get_tokens))
        .route("/exists", post(check_email))
        .route("/verify", post(send_email_verification))
        .route("/new", post(register_account))
        .route("/login", post(login));
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

#[derive(Deserialize)]
pub struct LoginOptions {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResult {
    pub access_token: String,
    pub user_id: String,
}

#[derive(Serialize)]
pub struct CheckMailRes {
    login_method: u8,
    new_account: bool,
}

#[derive(Serialize)]
pub struct PartialUser {
    email: String,
    name: String,
    uuid: String,
    flags: u8,
}

#[derive(Serialize)]
pub struct EmailVerificationResult {
    error: bool,
}
