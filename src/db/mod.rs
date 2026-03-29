use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

use crate::result::EchoResult;

pub async fn init_db(path: &str) -> EchoResult<SqlitePool> {
    let options = SqliteConnectOptions::from_str(path)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}
