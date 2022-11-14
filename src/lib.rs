use base64::decode;

pub fn decode_image_data(base64: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(decode(&base64)?)
}
