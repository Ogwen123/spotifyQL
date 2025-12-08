use std::time::SystemTime;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use ring::digest::{digest, SHA256};
use ring::rand::SecureRandom;
use crate::config::app_config::AppContext;
use crate::utils::logger::{warning};
use crate::utils::url::build_url;

pub fn sha256(code: String) -> Result<String, String> {
    let sha = digest(&SHA256, code.as_bytes());

    let hash;

    match str::from_utf8(sha.as_ref()) {
        Ok(res) => hash = res.to_string(),
        Err(err) => {
            warning!("{}", err.to_string());
            return Err("Invalid byte in sha digest, see above error".to_string())
        }
    }

    Ok(hash)
}

pub fn code_verifier() -> String {
    let binding = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
    let possible = binding.split("").collect::<Vec<&str>>();

    let mut values = String::new();

    let mut buf = vec![0; 128];

    let rng = ring::rand::SystemRandom::new();
    rng.fill(&mut buf).expect("Could not rng buffer in code verifier.");

    for num in buf {
        values += possible[num as usize];
    }

    values
}

pub fn login(cx: &mut AppContext) -> Result<(), String> {
    let code = code_verifier();
    let hash = sha256(code).map_err(|x| x)?;

    let b64 = BASE64_STANDARD.encode(hash);

    /* scopes
    playlist-read-private
    user-library-read
    user-follow-read
    */

    let url = build_url("https://accounts.spotify.com/authorize", vec![("response_type", "code")]);

    //if webbrowser::open()

    Ok(())
}
