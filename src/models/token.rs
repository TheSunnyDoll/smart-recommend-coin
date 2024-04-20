use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct Token {
    pub id: i64,
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_icon: String,
    pub holders: i64,
    pub created: i64,
    pub updated: i64,
    pub deleted: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTokenParams {
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_icon: String,
    pub holders: i64,
}

/// add token
pub async fn add(pool: &DbPool, token: AddTokenParams) -> Result<Option<Token>> {
    // check if exists
    if exists(pool, &token.token_address).await? {
        return Ok(None);
        // return Err(anyhow!("token already exists"));
    }
    let timestamp = chrono::Local::now().timestamp();
    let last_id = sqlx::query("INSERT INTO tokens (token_address, token_name, token_symbol, token_icon, holders, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(token.token_address)
        .bind(token.token_name)
        .bind(token.token_symbol)
        .bind(token.token_icon)
        .bind(token.holders)
        .bind(timestamp)
        .bind(timestamp)
        .execute(&pool.0)
        .await?
        .last_insert_rowid();
    Ok(Some(get(pool, last_id).await?))
}
/// exists token
pub async fn exists(pool: &DbPool, token_address: &str) -> Result<bool> {
    let query_res = sqlx::query("SELECT * FROM tokens WHERE token_address = ?")
        .bind(token_address)
        .fetch_one(&pool.0)
        .await;
    match query_res {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(err) => Err(err)?,
    }
}
/// get token
pub async fn get(pool: &DbPool, id: i64) -> Result<Token> {
    let item = sqlx::query_as("SELECT * FROM tokens WHERE id = ?")
        .bind(id)
        .fetch_one(&pool.0)
        .await?;
    Ok(item)
}
/// get token by address
pub async fn get_by_address(pool: &DbPool, address: &str) -> Result<Token> {
    let item = sqlx::query_as("SELECT * FROM tokens WHERE token_address = ?")
        .bind(address)
        .fetch_one(&pool.0)
        .await?;
    Ok(item)
}

/// top holders
pub async fn get_tokens(pool: &DbPool) -> Result<Vec<Token>> {
    let items = sqlx::query_as("SELECT * FROM tokens group by token_address order by holders desc")
        .fetch_all(&pool.0)
        .await?;
    Ok(items)
}
