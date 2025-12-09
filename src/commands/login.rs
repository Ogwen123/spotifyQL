use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::thread;
use crate::config::app_config::AppContext;
use crate::utils::logger::fatal;
use crate::utils::url::{build_url, parameterise_list};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use ring::digest::{Digest, SHA256, digest};
use ring::rand::SecureRandom;
use tokio::runtime::Runtime;
use warp::Filter;

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

    let (tx, rx) = channel::<String>();

    // open webserver to listen for auth response
    thread::spawn(move || {
        let rt = Runtime::new().expect("Could not init tokio runtime");
        rt.block_on(async move {
            let redirect_listener = warp::filters::query::raw()
                .map(move |params: String| {
                    if let Err(err) = tx.send(params.clone()) {
                        println!("{}", err);
                        fatal!("Could not send params to channel, see reason above")
                    }
                    params
                });

            warp::serve(redirect_listener).run(([127, 0, 0, 1], 5907)).await;
        });
    });

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
            return Err(
                "Could not open spotify authentication url in browser, see error reason below"
                    .to_string(),
            );
        }
    }

    // wait for auth code
    let mut params = rx.recv().expect("Web server thread stopped unexpectedly");

    //extract code

    // params will have a code param and could have a state param
    if params.contains("&") {
        params = params.split("&").collect::<Vec<&str>>()[0].to_string();
    }

    if !params.starts_with("code=") {
        return Err("Invalid redirect parameters".to_string())
    }

    let code = params.split("code=").collect::<Vec<&str>>()[1].to_string();



    Ok(())
}
