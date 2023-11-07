use serde::Serialize;
use sqlx::{MySql, Pool};
use std::env;

use utils::{consts::SENDER_EMAIL, errors::ApiError};

pub fn check_for_valid_email_address(email: &str) -> Result<(), ApiError> {
    // check email is legit (not spam or 10 min mail)
    match mailchecker::is_valid(email) {
        true => Ok(()),
        false => Err(ApiError {
            status: 400,
            error: true,
            message: "Invalid email address".to_string(),
        }),
    }
}

pub async fn add_db_email_verification(
    email: &str,
    code: &str,
    database: &Pool<MySql>,
) -> Result<(), ApiError> {
    // add auth code to db
    let db_req = sqlx::query!("INSERT INTO email_verification (email, code, expires) VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 1 HOUR))", email, code)
        .execute(database)
        .await;

    match db_req {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Internal Error".to_string(),
        }),
    }
}

pub async fn verify_email(email: &str, code: &str, database: &Pool<MySql>) -> Result<(), ApiError> {
    // check db for auth code
    let db_req = sqlx::query!(
        "SELECT email FROM email_verification WHERE email = ? AND code = ? AND expires > NOW()",
        email,
        code
    )
    .fetch_one(database)
    .await;

    match db_req {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(ApiError {
            status: 403,
            error: true,
            message: "Invalid code".to_string(),
        }),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Invalid code".to_string(),
        }),
    }
}

pub async fn send_verification_email(email: &str, code: &str, name: &str) -> Result<(), ApiError> {
    // prepare message body
    let email_body = format!(
        "Hey {name}! Your verification code for pastel.tv is {code}",
        name = name,
        code = code
    );

    // generate resend api call
    let email_message = ResendEmailData {
        from: SENDER_EMAIL.to_string(),
        to: email.to_string(),
        subject: "Verify your pastel.tv account".to_string(),
        text: email_body,
    };

    // send resend request to send email
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.resend.com/emails")
        .bearer_auth(&env::var("RESEND_KEY").expect("Missing RESEND_KEY"))
        .json(&email_message)
        .send()
        .await;

    match resp {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Internal Error".to_string(),
        }),
    }
}

pub async fn test_beta_code(code: &str, database: &Pool<MySql>) -> Result<(), ApiError> {
    // test db is beta code is valid
    let db_req = sqlx::query!(
        "SELECT code FROM beta_codes WHERE code = ? AND revoked = FALSE",
        code
    )
    .fetch_one(database)
    .await;

    match db_req {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(ApiError {
            status: 403,
            error: true,
            message: "Invalid beta code".to_string(),
        }),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Invalid beta code".to_string(),
        }),
    }
}

pub async fn add_beta_code(code: &str, database: &Pool<MySql>) -> Result<(), ApiError> {
    // add a beta access code for email registration to the DB
    let db_req = sqlx::query!("INSERT INTO beta_codes (code) VALUES (?)", code)
        .execute(database)
        .await;

    match db_req {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Internal Error".to_string(),
        }),
    }
}

pub async fn revoke_beta_code(code: &str, database: &Pool<MySql>) -> Result<(), ApiError> {
    // mark a beta code as revoked
    let db_req = sqlx::query!("UPDATE beta_codes SET revoked = TRUE WHERE code = ?", code)
        .execute(database)
        .await;

    match db_req {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Internal Error".to_string(),
        }),
    }
}

#[derive(Serialize)]
pub struct ResendEmailData {
    from: String,
    to: String,
    subject: String,
    text: String,
}
