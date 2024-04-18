use std::path::PathBuf;

pub mod capture_address;
pub mod db;
pub mod gmgn;
pub mod models;

const DATA_DIR: &str = "data";
pub fn get_data_dir() -> PathBuf {
    PathBuf::from(DATA_DIR)
}
