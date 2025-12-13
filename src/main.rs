use crate::commands::input::input_loop;
use crate::config::app_config::AppContext;
use crate::utils::logger::fatal;
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

    let mut cx = AppContext::new();

    if rc.command == Command::Login {
        if let Err(err) = login(&mut cx) {
            fatal!("{}", err)
        }
    } else if rc.command == Command::Logout {
        logout();
    } else if rc.command == Command::CLI {
        if let Err(err) = input_loop() {
            fatal!("{}", err)
        }
    }
}
