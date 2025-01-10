use base64::prelude::{Engine as _, BASE64_STANDARD};

pub fn from_base64(base64: &str) -> Vec<u8> {
  BASE64_STANDARD.decode(base64).unwrap()
}

pub fn jsonify(data: &impl serde::Serialize) -> String {
    serde_json::to_string(data)
        .unwrap()
        .trim_end() // Remove trailing newline
        .to_string()
}
