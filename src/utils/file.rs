use std::env::home_dir;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
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

    let path = file.path().map_err(|e| e)?;

    let _ = create_dir_all(path.clone()).map_err(|e| e.to_string());
    
    println!("{}", path.display());
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|_| {
            return "Could not open file in write mode (write, create, truncate).";
        })?;

    file.write_all(content.as_bytes()).map_err(|e| e.to_string()).map_err(|x| x.to_string());
    Ok(())
}

pub fn read_file(file: File) -> String {
    String::new()
}