use crate::app_context::AppContext;
use crate::auth::auth_listener::redirect_listener;
use crate::auth::code::{
    b64, code_verifier, create_file_content, fetch_access_token, parse_access_token_res, sha256,
};
use crate::utils::file::File;
use crate::utils::file::WriteMode::Overwrite;
use crate::utils::file::write_file;
use crate::utils::logger::{info, info_nnl, success};
use crate::utils::url::{build_url, parameterise_list};
use std::io;
use std::io::Write;
use std::sync::mpsc::channel;

/// Login to spotify using the PKCE auth flow
pub fn login(cx: &mut AppContext) -> Result<(), String> {
    let mut stdout = io::stdout();
    info!("Logging in");
    info_nnl!("Generating code challenge.");
    let code_verifier = code_verifier();
    let hash = sha256(code_verifier.clone())?;

    let code_challenge = b64(hash);

    success!("Generated code challenge.");
    stdout.flush().unwrap();

    let (tx, rx) = channel::<String>();

    redirect_listener(tx);

    /* scopes
    playlist-read-private - read all of a users playlists
    user-library-read - gives access to saved content
    user-follow-read - check if current user follows certain artist or user, get followed artists
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
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
            return Err(
                "Could not open spotify authentication url in browser, see error reason below"
                    .to_string(),
            );
        }
    }

    // wait for auth code
    info_nnl!("Listening for auth code response.");
    let mut params = rx.recv().expect("Web server thread stopped unexpectedly");
    success!("Received auth code response.");
    stdout.flush().unwrap();
    //extract code

    // params will have a code param and could have a state param
    if params.contains("&") {
        params = params.split("&").collect::<Vec<&str>>()[0].to_string();
    }

    if !params.starts_with("code=") {
        return Err("Invalid redirect parameters".to_string());
    }

    let code = params.split("code=").collect::<Vec<&str>>()[1].to_string();

    info_nnl!("Fetching access token.");
    let (tx, rx) = channel::<Result<String, String>>();

    fetch_access_token(
        tx,
        cx,
        code.clone(),
        code_verifier.clone(),
        redirect.to_string(),
    );

    let access_token_res = rx
        .recv()
        .expect("Access token request thread stopped unexpectedly.")?;
    success!("Received access token response.");
    stdout.flush().unwrap();
    info_nnl!("Parsing access token response.");
    let access_token_data = parse_access_token_res(access_token_res)?;
    success!("Parsed access token response.");
    stdout.flush().unwrap();
    write_file(
        File::Auth,
        create_file_content(access_token_data.clone())?,
        Overwrite,
    )?;

    cx.token = access_token_data.access_token;

    Ok(())
}
