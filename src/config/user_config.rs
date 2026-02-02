use serde::Deserialize;
use crate::utils::file::{read_file, File};
use crate::utils::logger::warning;

#[derive(Default, Clone)]
pub struct UserConfig {
    pub debug: bool
}

#[derive(Deserialize)]
struct ConfigFileContent {
    debug: bool
}

impl UserConfig {
    pub fn load() -> Result<UserConfig, String> {
        let mut cx = Self::default();

        let config_file_contents = match read_file(File::Config) {
            Ok(res) => res,
            Err(_) => {
                warning!("Could not load user config, resorting to default");
                return Ok(Self::default())
            },
        };

        let user_config: ConfigFileContent =
            serde_json::from_str(config_file_contents.as_str()).map_err(|x| x.to_string())?;

        cx.debug = user_config.debug;

        Ok(cx)
    }
}
