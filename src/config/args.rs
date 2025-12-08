use std::env;

#[derive(PartialEq)]
pub enum Command { 
    Login,
    Logout,
    CLI,
}

pub struct RunContext {
    pub command: Command,
}

impl RunContext {
    pub fn new() -> Self {
        let mut args = env::args().collect::<Vec<String>>();

        args.retain(|x| x != "spotifyQL"); // remove the binary's name from the args

        let command: Command;

        if args[0] == "login" {
            command = Command::Login
        } else if args[1] == "logout" {
            command = Command::Logout
        } else {
            // otherwise ignore args and just enter CLI
            command = Command::CLI
        }

        Self { command }
    }
}
