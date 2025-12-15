use crate::commands::input::input_loop;
use crate::config::app_config::AppContext;
use crate::utils::logger::{fatal, warning};
use crate::{
    commands::{login::login, logout::logout},
    config::args::{Command, RunContext},
};
use tokio::runtime::Runtime;

mod api;
mod auth;
mod commands;
mod config;
mod query;
mod utils;

fn main() {
    let rc = RunContext::new();

    let mut cx = match AppContext::new() {
        Ok(res) => res,
        Err(err) => {
            fatal!("{}", err);
            return
        }
    };

    if rc.command == Command::Login {
        if let Err(err) = login(&mut cx) {
            fatal!("{}", err)
        }
    } else if rc.command == Command::Logout {
        logout();
    } else if rc.command == Command::CLI {
        if cx.code.len() == 0 {
            warning!("You are not logged in and are being automatically sent to the login flow.");

            if let Err(err) = login(&mut cx) {
                fatal!("{}", err);
                return
            }
        }

        if let Err(err) = input_loop(&mut cx) {
            fatal!("{}", err)
        }
    }
}
