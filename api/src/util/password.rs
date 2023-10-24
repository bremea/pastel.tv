use bcrypt::{hash, verify};

use super::errors::ApiError;

pub async fn hash_password(password: &str) -> Result<String, ApiError> {
    let hashed = hash(password, 13);
    match hashed {
        Ok(hashed_password) => Ok(hashed_password),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Internal Error".to_string(),
        }),
    }
}

pub async fn verify_password(password: &str, hashed_password: &str) -> Result<bool, ApiError> {
    match verify(password, &hashed_password) {
        Ok(val) => Ok(val),
        Err(_) => Err(ApiError {
            status: 500,
            error: true,
            message: "Internal error".to_string(),
        }),
    }
}
