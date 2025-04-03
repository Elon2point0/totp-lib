use chrono::Utc;
use data_encoding::BASE32;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_code(secret_key: &str) -> String {
    let mut moving_factor = Utc::now().timestamp() / 30;

    let mut text: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    for i in (0..8).rev() {
        text[i] = (moving_factor & 0xFF) as u8;
        moving_factor >>= 8;
    }

    type HmacSha1 = Hmac<Sha1>;
    let decoded_key = BASE32.decode(secret_key.as_bytes()).unwrap();
    let mut mac = HmacSha1::new_from_slice(&decoded_key)
        .expect("HMAC can take key of any size");
    mac.update(&text);
    let hash = mac.finalize().into_bytes();
    let hash = hash.as_slice();

    let offset: usize = (hash[hash.len() - 1] & 0xF).into();

    let word: u32 =
        ((hash[offset] & 0x7F) as u32) << 24 |
        ((hash[offset + 1] & 0xFF) as u32) << 16 |
        ((hash[offset + 2] & 0xFF) as u32) << 8 |
        (hash[offset + 3] as u32) & 0xFF;

    let code_length = 6;
    let length = 10u32.pow(code_length);
    let otp = word as u32 % length;

    format!("{:06}", otp)
}