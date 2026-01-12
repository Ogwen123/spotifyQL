use crate::auth::token_refresh::refresh_token;
use crate::commands::input::input_loop;
use crate::utils::logger::{fatal, info_nnl, success, warning};
use crate::utils::utils::secs_now;
use crate::{
    commands::{login::login, logout::logout},
    config::args::{Command, RunContext},
};
use app_context::AppContext;
mod api;
mod app_context;
mod auth;
mod commands;
mod config;
mod query;
mod utils;

fn main() {
    let rc = RunContext::new();

    if rc.command == Command::Logout {
        if let Err(err) = logout() {
            fatal!("{}", err);
        }
        return;
    }

    let mut cx = match AppContext::new() {
        Ok(res) => res,
        Err(err) => {
            warning!("{}", err);
            fatal!("Could not load app context, run 'spotifyQL logout' before trying again.");
            return;
        }
    };

    if rc.command == Command::Login {
        if let Err(err) = login(&mut cx) {
            fatal!("{}", err)
        }
    } else if rc.command == Command::CLI {
        if cx.token.len() == 0 {
            warning!("You are not logged in and are being automatically sent to the login flow.");

            info_nnl!("Logging out.");
            if let Err(err) = logout() {
                fatal!("{}", err);
            }
            success!("Logged out.");

            if let Err(err) = login(&mut cx) {
                fatal!("{}", err);
                return;
            }
        }

        if secs_now() > cx.expires_after {
            info_nnl!("Refreshing token.");
            if let Err(err) = refresh_token(&mut cx) {
                fatal!("{}", err)
            }
            success!("Refreshed token.");
        }

        if let Err(err) = input_loop(&mut cx) {
            fatal!("{}", err)
        }
    }
}
