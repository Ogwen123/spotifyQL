use crate::config::app_config::AppContext;
use crate::utils::logger::{fatal, warning};
use crate::utils::url::{build_url, parameterise_list};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use ring::digest::{SHA256, digest, Digest};
use ring::rand::SecureRandom;
use std::time::SystemTime;

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

/// Login to spotify using the PKCE auth flow
pub fn login(cx: &mut AppContext) -> Result<(), String> {
    let code = code_verifier();
    let hash = sha256(code).map_err(|x| x)?;

    let code_challenge = BASE64_STANDARD.encode(hash);

    /* scopes
    playlist-read-private
    user-library-read
    user-follow-read
    */

    let scope = vec![
        "playlist-read-private",
        "user-library-read",
        "user-follow-read",
    ];
    let redirect = "http://127.0.0.1:5907";

    let url = build_url(
        "https://accounts.spotify.com/authorize",
        vec![
            ("response_type", "code"),
            ("client_id", cx.client_id.as_str()),
            ("scope", parameterise_list(scope).as_str()),
            ("code_challenge_method", "S256"),
            ("code_challenge", code_challenge.as_str()),
            ("redirect_uri", redirect),
        ],
    );

    match open::that(url.as_str()) {
        Ok(_) => {
            println!("{}", url);
            println!("opened")
        }
        Err(err) => {
            println!("{}", err);
            return Err("Could not open spotify authentication url in browser, see error reason below".to_string())
        }
    }



    Ok(())
}
