use crate::app_context::AppContext;
use crate::utils::logger::{info, success};
use crate::utils::utils::secs_now;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use reqwest::StatusCode;
use ring::digest::{Digest, SHA256, digest};
use ring::rand::SecureRandom;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use std::thread;
use tokio::runtime::Runtime;

#[derive(Serialize)]
pub struct AccessTokenRequestParams {
    pub grant_type: String,
    pub code: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub code_verifier: String,
}

pub fn b64(hash: Digest) -> String {
    BASE64_STANDARD
        .encode(hash)
        .replace("=", "")
        .replace("+", "-")
        .replace("/", "_")
}

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
pub struct AuthFileContent {
    pub token: String,
    pub refresh_token: String,
    pub expires_after: u64,
}

pub fn create_file_content(atd: AccessTokenResponse) -> Result<String, String> {
    Ok(serde_json::to_string(&AuthFileContent {
        token: atd.access_token,
        refresh_token: atd.refresh_token,
        expires_after: secs_now() + atd.expires_in,
    })
    .map_err(|x| x.to_string())?)
}

pub fn fetch_access_token(
    tx: Sender<Result<String, String>>,
    _cx: &AppContext,
    code: String,
    code_verifier: String,
    redirect: String,
) {
    let cx = _cx.clone();
    // request access token
    thread::spawn(move || {
        let rt = Runtime::new().expect("Could not init tokio runtime");
        rt.block_on(async move {
            let body = match serde_urlencoded::to_string(AccessTokenRequestParams {
                grant_type: "authorization_code".to_string(),
                client_id: cx.client_id,
                code,
                code_verifier,
                redirect_uri: redirect,
            }) {
                Ok(res) => res,
                Err(err) => {
                    tx.send(Err(err.to_string()))
                        .expect("Failed to send error down request channel. (3)");
                    return;
                }
            };

            let client = reqwest::Client::new();

            let resp_result = client
                .post("https://accounts.spotify.com/api/token")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await;

            let resp = match resp_result {
                Ok(res) => res,
                Err(err) => {
                    tx.send(Err(err.to_string()))
                        .expect("Failed to send error down request channel. (4)");
                    return;
                }
            };
            let status = resp.status();
            let body_result = resp.text().await;

            let body = match body_result {
                Ok(res) => res,
                Err(err) => {
                    tx.send(Err(err.to_string()))
                        .expect("Failed to send error down request channel. (6)");
                    return;
                }
            };

            if status != StatusCode::OK {
                tx.send(Err(format!(
                    "Received error from access token request. \n{}",
                    body
                )
                .to_string()))
                    .expect("Failed to send error down request channel. (5)");
                return;
            }

            tx.send(Ok(body))
                .expect("Failed to send success response down request channel. (2)");
        });
    });
}

#[derive(Deserialize, Clone)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

pub fn parse_access_token_res(res: String) -> Result<AccessTokenResponse, String> {
    let res: AccessTokenResponse = serde_json::from_str(res.as_str()).map_err(|x| x.to_string())?;

    Ok(res)
}
