use crate::auth::code::{
    AccessTokenRequestParams, AccessTokenResponse, create_file_content, parse_access_token_res,
};
use crate::config::app_config::AppContext;
use crate::utils::file::WriteMode::Overwrite;
use crate::utils::file::{File, write_file};
use crate::utils::utils::secs_now;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::channel;
use std::thread;
use tokio::runtime::Runtime;

#[derive(Serialize)]
pub struct RefreshTokenRequestParams {
    grant_type: String,
    refresh_token: String,
    client_id: String,
}

pub fn refresh_token(_cx: &mut AppContext) -> Result<(), String> {
    // request token refresh

    let (tx, rx) = channel::<Result<String, String>>();

    let cx = _cx.clone();
    thread::spawn(move || {
        let rt = Runtime::new().expect("Could not init tokio runtime");
        rt.block_on(async move {
            let body = match serde_urlencoded::to_string(RefreshTokenRequestParams {
                grant_type: "refresh_token".to_string(),
                refresh_token: cx.refresh_token,
                client_id: cx.client_id,
            }) {
                Ok(res) => res,
                Err(err) => {
                    tx.send(Err(err.to_string()))
                        .expect("Failed to send error down request channel. (7)");
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
                        .expect("Failed to send error down request channel. (8)");
                    return;
                }
            };
            let status = resp.status();
            let body_result = resp.text().await;

            let body = match body_result {
                Ok(res) => res,
                Err(err) => {
                    tx.send(Err(err.to_string()))
                        .expect("Failed to send error down request channel. (9)");
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
                .expect("Failed to send success response down request channel. (3)");
        });
    });

    let res = rx
        .recv()
        .expect("Refresh token request thread stopped unexpectedly.")?;

    let refresh_token_data = parse_access_token_res(res)?;

    write_file(
        File::Auth,
        create_file_content(refresh_token_data.clone())?,
        Overwrite,
    )?;

    _cx.token = refresh_token_data.access_token;
    _cx.refresh_token = refresh_token_data.refresh_token;
    _cx.expires_after = secs_now() + refresh_token_data.expires_in;

    Ok(())
}
