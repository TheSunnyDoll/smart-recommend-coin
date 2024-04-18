use std::path::PathBuf;

pub mod capture;
pub mod db;
pub mod models;
pub mod services;

const DATA_DIR: &str = "data";
pub fn get_data_dir() -> PathBuf {
    PathBuf::from(DATA_DIR)
}
