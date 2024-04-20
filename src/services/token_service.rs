use std::error::Error;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    capture::Capture,
    db, models,
    models::address_token::{self, CaptureResponseData},
};

pub const CAPTURE_FILE: &str = "data/capture_address.json";

pub fn capture_get(capture: impl Capture) -> Result<CaptureResponseData, Box<dyn Error>> {
    let data = capture.get()?;
    Ok(serde_json::from_str(&data)?)
}

pub async fn tokens(State(pool): State<db::DbPool>) -> impl IntoResponse {
    let items = models::token::get_tokens(&pool).await.unwrap();
    Json(items)
}

pub async fn stat_token_holder(pool: db::DbPool) -> Result<(), Box<dyn Error>> {
    let items = address_token::get_token_by_holders(&pool).await.unwrap();
    for item in items {
        let token_params = models::token::AddTokenParams {
            token_icon: item.token_icon,
            token_name: item.token_name,
            token_symbol: item.token_symbol,
            token_address: item.token_address,
            holders: item.holders,
        };
        models::token::add(&pool, token_params).await?;
    }
    Ok(())
}
