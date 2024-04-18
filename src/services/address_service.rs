use std::error::Error;

use crate::{
    capture::{read_from_file, Capture, CAPTURE_ADDRESS_FILE},
    models::address::CaptureResponseData,
};

pub fn capture_save(capture: impl Capture) -> Result<(), Box<dyn Error>> {
    capture.save()
}

pub fn read_capture_data() -> Result<CaptureResponseData, Box<dyn Error>> {
    let data = read_from_file(CAPTURE_ADDRESS_FILE)?;
    Ok(serde_json::from_str(&data)?)
}
