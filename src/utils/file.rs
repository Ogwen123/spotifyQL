use crate::app_context::AppContext;
use crate::query::data::KeyAccess;
use crate::utils::utils::iso_str;
use std::env::home_dir;
use std::fs::{OpenOptions, create_dir_all, remove_file};
use std::io::{Read, Write};
use std::path::PathBuf;

pub enum File {
    Auth,
    Config,
    Other(String),
}

impl File {
    fn folder_path() -> Result<PathBuf, String> {
        let mut home = match home_dir() {
            Some(res) => res,
            None => {
                return Err("Could not get home directory.".to_string());
            }
        };
        let os = std::env::consts::OS;

        if os == "linux" || os == "macos" {
            home.push(".config");
            home.push("spotifyQL");
        } else if os == "windows" {
            home.push("AppData");
            home.push("Local");
            home.push("spotifyQL")
        } else {
            return Err(String::from("Unsupported OS"));
        }

        Ok(home)
    }

    fn path(&self) -> Result<PathBuf, String> {
        let mut folder = File::folder_path()?;

        match self {
            File::Auth => folder.push("auth.json"),
            File::Config => folder.push("config.json"),
            File::Other(res) => return Ok(PathBuf::from(res)),
        }

        Ok(folder)
    }

    fn create_parent() -> Result<(), String> {
        create_dir_all(File::folder_path()?).map_err(|e| e.to_string())
    }
}

#[derive(PartialEq)]
pub enum WriteMode {
    Overwrite,
    Append,
}

pub fn write_file(file: File, content: String, write_mode: WriteMode) -> Result<(), String> {
    let path = file.path().map_err(|e| e)?;

    File::create_parent()?; // make sure the parent folders exist

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(if write_mode == WriteMode::Overwrite {
            true
        } else {
            false
        })
        .open(path)
        .map_err(|_| {
            return "Could not open file in write mode (write, create, truncate).";
        })?;

    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn read_file(file: File) -> Result<String, String> {
    let path = file.path().map_err(|e| e)?;

    let mut file = OpenOptions::new().read(true).open(path).map_err(|_| {
        return "Could not open file in read mode (read).";
    })?;

    let mut content: String = String::new();
    file.read_to_string(&mut content)
        .map_err(|x| x.to_string())?;
    Ok(content)
}

pub fn delete_file(file: File) -> Result<(), String> {
    let path = file.path().map_err(|e| e)?;

    if path.exists() {
        remove_file(path).map_err(|x| x.to_string())?
    }

    Ok(())
}

fn csv<T>(data: T) -> String
where
    T: KeyAccess,
{
    let mut res: Vec<String> = Vec::new();
    for attr in T::attributes() {
        res.push(data.access(attr).unwrap().to_string()) // attr is guaranteed to be an attribute because of where it comes from so this unwrap is safe
    }

    res.join(",")
}

/// Output format
///
/// query \n
/// iso time \n
/// line1 \n
/// line2 \n
/// etc
pub fn write_result<T>(cx: &AppContext, valid: Vec<T>, query: String) -> Result<(), String>
where
    T: KeyAccess,
{
    if cx.save_file.is_none() {
        return Err("No save file specified".to_string());
    }
    let mut output: Vec<String> = Vec::new();

    output.push(query);
    output.push(iso_str());

    for i in valid {
        output.push(csv(i));
    }

    write_file(
        File::Other(cx.save_file.clone().unwrap()),
        output.join("\n"),
        WriteMode::Overwrite,
    )
}
