use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

// capture struct

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureToken {
    pub token_address: Option<String>,
    pub amount: i64,
    pub decimals: u32,
    pub owner: String,
    pub token_icon: Option<String>,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub price_usdt: Option<f64>,
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CaptureData {
    pub data_type: String,
    pub count: u32,
    pub tokens: Vec<CaptureToken>,
}

#[derive(Debug, Deserialize)]
pub struct CaptureResponseData {
    pub success: bool,
    pub data: CaptureData,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct Token {
    pub id: i64,
    pub owner: String,
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_icon: String,
    pub amount: String,
    pub price_usdt: f64,
    pub created: i64,
    pub updated: i64,
    pub deleted: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTokenParams {
    pub owner: String,
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_icon: String,
    pub amount: String,
    pub price_usdt: f64,
}

impl From<CaptureToken> for AddTokenParams {
    fn from(v: CaptureToken) -> Self {
        Self {
            owner: v.owner,
            token_address: v.token_address.unwrap_or_default(),
            token_name: v.token_name.unwrap_or_default(),
            token_symbol: v.token_symbol.unwrap_or_default(),
            token_icon: v.token_icon.unwrap_or_default(),
            amount: v.amount.to_string(),
            price_usdt: v.price_usdt.unwrap_or_default(),
        }
    }
}

/// add token
pub async fn add(pool: &DbPool, token: AddTokenParams) -> Result<Token> {
    // check if exists
    if exists(pool, &token.owner, &token.token_address).await? {
        return Err(anyhow!("token already exists"));
    }
    let timestamp = chrono::Local::now().timestamp();
    let last_id = sqlx::query("INSERT INTO tokens (owner, token_address, token_name, token_symbol, token_icon, amount, price_usdt, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(token.owner)
        .bind(token.token_address)
        .bind(token.token_name)
        .bind(token.token_symbol)
        .bind(token.token_icon)
        .bind(token.amount)
        .bind(token.price_usdt)
        .bind(timestamp)
        .bind(timestamp)
        .execute(&pool.0)
        .await?
        .last_insert_rowid();
    get(pool, last_id).await
}
/// exists token
pub async fn exists(pool: &DbPool, owner: &str, token_address: &str) -> Result<bool> {
    let query_res = sqlx::query("SELECT * FROM tokens WHERE owner = ? and token_address = ?")
        .bind(owner)
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
    let book = sqlx::query_as("SELECT * FROM tokens WHERE id = ?")
        .bind(id)
        .fetch_one(&pool.0)
        .await?;
    Ok(book)
}
