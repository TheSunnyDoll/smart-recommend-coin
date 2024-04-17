use std::{fs::File, io::Write};

use crate::capture_address::{self, Capture, ResponseData};

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
    fn get(&self) -> Result<ResponseData, reqwest::Error> {
        reqwest::blocking::get(self.url.clone())?.json()
    }

    fn save(&self) -> Result<(), reqwest::Error> {
        let text = reqwest::blocking::get(self.url.clone())?.text()?;
        let mut file = File::create(capture_address::CAPTURE_FILE).unwrap();
        file.write_all(text.as_bytes()).unwrap();
        println!(
            "Capture address json saved to {:?}",
            capture_address::CAPTURE_FILE
        );
        Ok(())
    }
}
