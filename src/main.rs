use crate::config::app_config::AppContext;
use crate::utils::logger::fatal;
use crate::{
    commands::{login::login, logout::logout},
    config::args::{Command, RunContext},
};
use tokio::runtime::Runtime;

mod auth;
mod commands;
mod config;
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
    } else {
        println!("command line WIP")
    }
}
