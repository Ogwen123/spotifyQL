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
    CLI,
}

pub struct RunContext {
    pub command: Command,
    pub ui_mode: UIMode,
    pub file_output: Option<String>,
}

impl RunContext {
    pub fn new() -> Self {
        let mut _args = env::args().collect::<Vec<String>>();
        println!("{:?}", _args);
        _args.retain(|x| !x.ends_with("spotifyQL") && !x.ends_with("spotifyQL.exe")); // remove the binary's name from the args
        println!("{:?}", _args);

        let mut args = _args.into_iter().peekable();

        let mut command: Command = Command::CLI;
        let mut ui_mode = UIMode::Default;
        let mut file_output: Option<String> = None;

        while args.peek().is_some() {
            let arg = args.next().unwrap();
            if arg == "login" {
                command = Command::Login
            } else if arg == "logout" {
                command = Command::Logout
            } else if arg == "--no-tui" {
                ui_mode = UIMode::CLI;
            } else if arg == "--tui" {
                ui_mode = UIMode::TUI
            } else if arg == "--file" {
                file_output = args.next();
            }
        }

        Self {
            command,
            ui_mode,
            file_output,
        }
    }
}
