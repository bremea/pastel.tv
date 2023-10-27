use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};
use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::{middleware::auth::sign_jwt, util::errors::ApiError};

pub async fn new_refresh_token(user_id: &str, database: &Pool<MySql>) -> Result<String, ApiError> {
    // generate token uuid
    let token_uuid = Uuid::new_v4().to_string();

    // hash uuid
    let mut hasher = Sha256::new();
    hasher.update(token_uuid);
    let result = hasher.finalize();
    let refresh_token = Base64::encode_string(&result);

    // add token to db
    let db_req = sqlx::query!(
        "INSERT INTO refresh_tokens (token, user_id, expires) VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 3 MONTH))",
		refresh_token,
        user_id
    )
    .execute(database)
    .await;

    match db_req {
        Ok(_) => Ok(refresh_token),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Internal error".to_string(),
        }),
    }
}

pub async fn login_with_refresh_token(
    value: &str,
    database: &Pool<MySql>,
) -> Result<AuthTokens, ApiError> {
    // check if refresh token is valid
    let find_refresh_token = sqlx::query!(
        "SELECT user_id FROM refresh_tokens WHERE token = ? AND revoked = FALSE AND expires > NOW()",
        value
    )
    .fetch_one(database)
    .await;

    // try to get user uuid from refresh token
    let user_id = match find_refresh_token {
        Ok(res) => res.user_id,
        Err(sqlx::Error::RowNotFound) => {
            return Err(ApiError {
                status: 403,
                error: true,
                message: "Invalid refresh token".to_string(),
            });
        }
        Err(_) => {
            return Err(ApiError {
                status: 500,
                error: true,
                message: "Internal error".to_string(),
            });
        }
    };

    // revoke this token
    revoke_refresh_token(value, database).await?;

    // generate new tokens
    let login_result = generate_tokens(user_id, database).await?;
    Ok(login_result)
}

pub async fn generate_tokens(
    user_id: String,
    database: &Pool<MySql>,
) -> Result<AuthTokens, ApiError> {
    // generate new refresh token
    let refresh_token = new_refresh_token(&user_id, database).await?;
    // generate new access token
    let access_token = sign_jwt(&user_id);

    Ok(AuthTokens {
        refresh_token,
        access_token,
        user_id,
    })
}

pub async fn revoke_refresh_token(value: &str, database: &Pool<MySql>) -> Result<(), ApiError> {
    // mark refresh token as revoked
    let db_req = sqlx::query!(
        "UPDATE refresh_tokens SET revoked = TRUE WHERE token = ? AND revoked = FALSE AND expires > NOW()",
        value
    )
    .execute(database)
    .await;

    match db_req {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Internal error".to_string(),
        }),
    }
}

pub struct AuthTokens {
    pub refresh_token: String,
    pub access_token: String,
    pub user_id: String,
}
