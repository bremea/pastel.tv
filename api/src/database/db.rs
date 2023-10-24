use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;

pub async fn connect() -> Result<Pool<MySql>, sqlx::Error> {
    let pool: Pool<MySql>  = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("Missing DATABASE_URL"))
        .await?;

    Ok(pool)
}
