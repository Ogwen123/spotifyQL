use std::env::home_dir;
use std::path::PathBuf;

pub enum File {
    Auth,
    Config
}

impl File {
    fn path(&self) -> Result<PathBuf, String> {
        let mut home = match home_dir() {
            Some(res) => res,
            None => {
                return Err("Could not get home directory.".to_string());
            }
        };
        let os = std::env::consts::OS;

        if os == "linux" || os == "macos"{
            home.push(".config");
            home.push("spotifyQL");
        } else if os == "windows" {
            home.push("AppData");
            home.push("Local");
            home.push("spotifyQL")
        } else {
            return Err(String::from("Unsupported OS"))
        }

        match self {
            File::Auth => home.push("auth.json"),
            File::Config => home.push("config.json")
        }

        Ok(home)
    }
}

pub enum WriteMode {
    Overwrite,
    Append
}

pub fn write_file(file: File, content: String, write_mode: WriteMode) -> Result<(), String> {
    Ok(())
}

pub fn read_file(file: File) -> String {
    String::new()
}