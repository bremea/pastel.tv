use bitfield_struct::bitfield;
use sqlx::{MySql, Pool};

use crate::util::errors::ApiError;

pub async fn get_user(email: &str, database: &Pool<MySql>) -> Result<Option<User>, ApiError> {
	// fetch user from db
    let db_req = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
        .fetch_one(database)
        .await;

    match db_req {
        Ok(user) => Ok(Some(user)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(_) => Err(ApiError {
			status: 500,
            error: true,
            message: "Internal Error".to_string(),
        }),
    }
}

pub async fn add_user(user: &User, database: &Pool<MySql>) -> Result<(), ApiError> {
	// add user to db
    let db_req = sqlx::query!(
        "INSERT INTO users (uuid, email, name, auth_method, password, flags) VALUES (?, ?, ?, ?, ?, ?)",
        user.uuid,
		user.email,
		user.name,
		user.auth_method,
		user.password,
		user.flags.0
    )
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

pub struct User {
    pub uuid: String,
    pub email: String,
    pub name: String,
    pub auth_method: u8,
    pub password: Option<String>,
    pub flags: UserFlags,
}

#[bitfield(u8)]
pub struct UserFlags {
    pub email_verified: bool,
    pub developer: bool,
    #[bits(6)]
    _reserved: usize,
}
