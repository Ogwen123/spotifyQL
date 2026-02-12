use std::env;

#[derive(PartialEq)]
pub enum Command {
    Login,
    Logout,
    CLI,
}

#[derive(PartialEq)]
pub enum UIMode {
    Default,
    TUI,
    CLI
}

pub struct RunContext {
    pub command: Command,
    pub ui_mode: UIMode
}

impl RunContext {
    pub fn new() -> Self {
        let mut args = env::args().collect::<Vec<String>>();
        println!("{:?}", args);
        args.retain(|x| !x.ends_with("spotifyQL") && !x.ends_with("spotifyQL.exe")); // remove the binary's name from the args
        println!("{:?}", args);

        let mut command: Command = Command::CLI;
        let mut ui_mode = UIMode::Default;

        for arg in args{
            if arg == "login" {
                command = Command::Login
            } else if arg == "logout" {
                command = Command::Logout
            } else if arg == "--no-tui" {
                ui_mode = UIMode::CLI;
            } else if arg == "--tui" {
                ui_mode = UIMode::TUI
            }
        }

        Self { command, ui_mode }
    }
}
