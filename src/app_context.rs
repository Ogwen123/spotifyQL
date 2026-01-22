use crate::auth::code::AuthFileContent;
use crate::query::data::Data;
use crate::utils::file::{File, read_file};

#[derive(Clone)]
pub struct AppContext {
    pub client_id: String,
    pub token: String,
    pub refresh_token: String,
    pub expires_after: u64,
    pub data: Data,
}

impl AppContext {
    pub fn new() -> Result<Self, String> {
        let mut cx = Self::default();

        let auth_file_contents = match read_file(File::Auth) {
            Ok(res) => res,
            Err(_) => return Ok(Self::default()),
        };

        let auth_data: AuthFileContent =
            serde_json::from_str(auth_file_contents.as_str()).map_err(|x| x.to_string())?;

        cx.token = auth_data.token;
        cx.refresh_token = auth_data.refresh_token;
        cx.expires_after = auth_data.expires_after;

        Ok(cx)
    }
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            client_id: "d46aab9576a9435593e70791f3cf70d7".to_string(),
            token: String::new(),
            refresh_token: String::new(),
            expires_after: 0,
            data: Default::default(),
        }
    }
}
