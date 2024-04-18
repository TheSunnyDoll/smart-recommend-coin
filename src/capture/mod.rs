use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod gmgn;
pub mod solscan;

pub trait Capture {
    fn get(&self) -> Result<String, Box<dyn Error>>;
    fn save(&self) -> Result<(), Box<dyn Error>>;
}

pub const CAPTURE_ADDRESS_FILE: &str = "data/capture_address.json";

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
