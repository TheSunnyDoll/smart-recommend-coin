use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite};

use crate::db::DbPool;

// capture struct
#[derive(Debug, Deserialize)]
pub struct CaptureRankData {
    pub wallet_address: String,
    pub address: String,
    pub realized_profit: Option<f64>,
    pub buy: u32,
    pub sell: u32,
    pub last_active: i64,
    pub realized_profit_7d: Option<f64>,
    pub pnl_30d: Option<f64>,
    pub pnl_7d: Option<f64>,
    pub pnl_1d: Option<f64>,
    pub txs_30d: u32,
    pub eth_balance: Option<f64>,
    pub sol_balance: Option<f64>,
    pub twitter_username: Option<String>,
    pub avatar: Option<String>,
    pub ens: Option<String>,
    pub tag: String,
    pub tag_rank: CaptureTagRank,
    pub nickname: Option<String>,
    pub tags: Vec<String>,
    pub twitter_name: Option<String>,
    pub followers_count: u32,
    pub is_blue_verified: bool,
    pub twitter_description: Option<String>,
    pub name: Option<String>,
    pub avg_hold_time: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CaptureTagRank {
    pub smart_degen: u32,
}

#[derive(Debug, Deserialize)]
pub struct CaptureData {
    pub rank: Vec<CaptureRankData>,
}

#[derive(Debug, Deserialize)]
pub struct CaptureResponseData {
    pub code: u32,
    pub msg: String,
    pub data: CaptureData,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct Address {
    pub id: i64,
    pub address: String,
    pub source: String,
    pub pnl_1d: f64,
    pub pnl_7d: f64,
    pub pnl_30d: f64,
    // 30 days
    pub realized_profit: f64,
    pub realized_profit_7d: f64,
    pub last_active: i64,
    pub created: i64,
    pub updated: i64,
    pub deleted: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAddressParams {
    pub address: String,
    pub source: String,
    pub pnl_1d: f64,
    pub pnl_7d: f64,
    pub pnl_30d: f64,
    pub realized_profit: f64,
    pub realized_profit_7d: f64,
    pub last_active: i64,
}
impl From<CaptureRankData> for AddAddressParams {
    fn from(v: CaptureRankData) -> Self {
        Self {
            address: v.address,
            source: "".to_string(),
            pnl_1d: v.pnl_1d.unwrap_or_default(),
            pnl_7d: v.pnl_7d.unwrap_or_default(),
            pnl_30d: v.pnl_30d.unwrap_or_default(),
            realized_profit: v.realized_profit.unwrap_or_default(),
            realized_profit_7d: v.realized_profit_7d.unwrap_or_default(),
            last_active: v.last_active,
        }
    }
}

/// add address
pub async fn add(pool: &DbPool, address: AddAddressParams) -> Result<Option<Address>> {
    // check if exists
    if exists(pool, &address.address).await? {
        return Ok(None);
    }
    let timestamp = chrono::Local::now().timestamp();
    let last_id = sqlx::query("INSERT INTO addresses (address, source, pnl_1d, pnl_7d, pnl_30d, realized_profit, realized_profit_7d, last_active, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(address.address)
        .bind(address.source)
        .bind(address.pnl_1d)
        .bind(address.pnl_7d)
        .bind(address.pnl_30d)
        .bind(address.realized_profit)
        .bind(address.realized_profit_7d)
        .bind(address.last_active)
        .bind(timestamp)
        .bind(timestamp)
        .execute(&pool.0)
        .await?
        .last_insert_rowid();
    Ok(Some(get(pool, last_id).await?))
}
/// exists address
pub async fn exists(pool: &DbPool, address: &str) -> Result<bool> {
    let query_res = sqlx::query("SELECT * FROM addresses WHERE address = ?")
        .bind(address)
        .fetch_one(&pool.0)
        .await;
    match query_res {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(err) => Err(err)?,
    }
}
/// get address
pub async fn get(pool: &DbPool, id: i64) -> Result<Address> {
    let book = sqlx::query_as("SELECT * FROM addresses WHERE id = ?")
        .bind(id)
        .fetch_one(&pool.0)
        .await?;
    Ok(book)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListParams {
    pub limit: Option<(i64, i64)>,
    pub kw: Option<String>,
}
/// list address
pub async fn list_all(pool: &DbPool) -> Result<Vec<Address>> {
    let mut query: QueryBuilder<Sqlite> =
        QueryBuilder::new("select * from addresses where deleted = 0");

    // query.push(" order by created DESC");
    // if let Some(limit) = filter.limit {
    //     query.push(format!(" limit {},{}", limit.0, limit.1));
    // }
    let items = query.build_query_as().fetch_all(&pool.0).await?;
    Ok(items)
}
