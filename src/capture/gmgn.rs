use std::{error::Error, fs::File, io::Write};

use crate::capture::CAPTURE_ADDRESS_FILE;

use super::Capture;

pub struct GmgnCapture {
    url: String,
}

impl GmgnCapture {
    pub fn new(chain: String, days: i8) -> Self {
        Self {
            url: format!("https://gmgn.ai/defi/quotation/v1/rank/{}/wallets/{}d?orderby=pnl_{}d&direction=desc&tag=smart_degen", chain, days, days)
        }
    }
}

impl Capture for GmgnCapture {
    fn get(&self) -> Result<String, Box<dyn Error>> {
        Ok(reqwest::blocking::get(self.url.clone())?.text()?)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let text = reqwest::blocking::get(self.url.clone())?.text()?;
        let mut file = File::create(CAPTURE_ADDRESS_FILE).unwrap();
        file.write_all(text.as_bytes()).unwrap();
        println!("Capture address json saved to {:?}", CAPTURE_ADDRESS_FILE);
        Ok(())
    }
}
