use std::{error::Error, fs::File, io::BufReader, path::Path};

use reqwest;
use serde::Deserialize;

pub const CAPTURE_FILE: &str = "data/capture_address.json";

pub trait Capture {
    fn get(&self) -> Result<ResponseData, reqwest::Error>;
    fn save(&self) -> Result<(), reqwest::Error>;
}

pub fn perform_capture(capture: impl Capture) -> Result<(), reqwest::Error> {
    capture.save()
}

pub fn read() -> Result<ResponseData, Box<dyn Error>> {
    read_from_file(CAPTURE_FILE)
}

fn read_from_file<P: AsRef<Path>>(path: P) -> Result<ResponseData, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let res = serde_json::from_reader(reader)?;
    Ok(res)
}
#[derive(Debug, Deserialize)]
pub struct RankData {
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
    pub tag_rank: TagRank,
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
pub struct TagRank {
    pub smart_degen: u32,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub rank: Vec<RankData>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseData {
    pub code: u32,
    pub msg: String,
    pub data: Data,
}
