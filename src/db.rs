use anyhow::Result;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tracing::debug;

use crate::get_data_dir;

pub struct DbPool(pub SqlitePool);

pub async fn init() -> Result<()> {
    let db_path = get_data_dir().join("main.db");
    debug!("init db: {}", db_path.display());

    let connect_options = SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename(db_path.to_str().unwrap());

    let pool = SqlitePool::connect_with(connect_options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}

// get sqlite pool
pub async fn get_pool() -> Result<DbPool> {
    let db_path = get_data_dir().join("main.db");
    debug!("get pool: {}", db_path.display());

    let connect_options = SqliteConnectOptions::new().filename(db_path.to_str().unwrap());

    let pool = SqlitePool::connect_with(connect_options).await?;
    Ok(DbPool(pool))
}

#[cfg(test)]
pub mod tests {
    use super::DbPool;
    pub async fn get_test_pool() -> DbPool {
        let sqlite_pool = sqlx::sqlite::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();
        let _ = sqlx::migrate!("./migrations")
            .run(&sqlite_pool)
            .await
            .unwrap();
        DbPool(sqlite_pool)
    }
}
