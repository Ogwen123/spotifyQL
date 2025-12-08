use std::env;
use crate::{
    commands::{login::login, logout::logout},
    config::args::{Command, RunContext},
};
use crate::config::app_config::AppContext;

mod auth;
mod commands;
mod config;
mod utils;

fn main() {
    let rc = RunContext::new();

    let mut cx = AppContext::new();

    if rc.command == Command::Login {
        login(&mut cx);
    } else if rc.command == Command::Logout {
        logout();
    }
}
