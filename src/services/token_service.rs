use std::error::Error;

use crate::{capture::Capture, models::token::CaptureResponseData};

pub const CAPTURE_FILE: &str = "data/capture_address.json";

pub fn capture_get(capture: impl Capture) -> Result<CaptureResponseData, Box<dyn Error>> {
    let data = capture.get()?;
    Ok(serde_json::from_str(&data)?)
}
