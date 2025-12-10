use crate::auth::auth_listener::redirect_listener;
use crate::auth::code::{code_verifier, create_file_content, sha256};
use crate::config::app_config::AppContext;
use crate::utils::file::File;
use crate::utils::file::WriteMode::Overwrite;
use crate::utils::file::write_file;
use crate::utils::url::{build_url, parameterise_list};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use std::sync::mpsc::channel;

/// Login to spotify using the PKCE auth flow
pub fn login(cx: &mut AppContext) -> Result<(), String> {
    let code = code_verifier();
    let hash = sha256(code).map_err(|x| x)?;

    let code_challenge = BASE64_STANDARD.encode(hash);

    let (tx, rx) = channel::<String>();

    redirect_listener(tx);

    /* scopes
    playlist-read-private - read all of a users playlists
    user-library-read - find all the playlists in a users library
    user-follow-read - read all the followers a user has
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
        return Err("Invalid redirect parameters".to_string());
    }

    let code = params.split("code=").collect::<Vec<&str>>()[1].to_string();

    write_file(
        File::Auth,
        create_file_content(code).map_err(|x| x)?,
        Overwrite,
    )
    .map_err(|x| x)?;

    Ok(())
}
