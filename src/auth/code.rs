use ring::digest::{Digest, SHA256, digest};
use ring::rand::SecureRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn sha256(code: String) -> Result<Digest, String> {
    let sha = digest(&SHA256, code.as_bytes());

    Ok(sha)
}

pub fn code_verifier() -> String {
    let binding = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
    let possible = binding.split("").collect::<Vec<&str>>();

    let pl: u8 = possible.len() as u8;

    let mut values = String::new();

    let mut buf = vec![0; 128];

    let rng = ring::rand::SystemRandom::new();
    rng.fill(&mut buf)
        .expect("Could not rng buffer in code verifier.");

    for num in buf {
        values += possible[(num % pl) as usize];
    }

    values
}

#[derive(Serialize, Deserialize)]
struct AuthFileContent {
    code: String,
}

pub fn create_file_content(code: String) -> Result<String, String> {
    Ok(serde_json::to_string(&AuthFileContent { code }).map_err(|x| x.to_string())?)
}
