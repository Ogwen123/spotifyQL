use crate::config::app_config::AppContext;
use crate::utils::logger::fatal;
use crate::{
    commands::{login::login, logout::logout},
    config::args::{Command, RunContext},
};
use tokio::runtime::Runtime;
use crate::commands::input::input_loop;

mod auth;
mod commands;
mod config;
mod utils;
mod api;
mod query;

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
        input_loop()
    }
}
