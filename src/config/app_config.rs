use crate::auth::code::AuthFileContent;
use crate::utils::file::{read_file, File};

pub struct PlaylistData {
    pub name: String,
}

pub struct AlbumData {
    pub name: String,
}

pub struct Data {
    pub playlist_data: Option<Vec<PlaylistData>>,
    pub album_data: Option<Vec<AlbumData>>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            playlist_data: None,
            album_data: None,
        }
    }
}

pub struct AppContext {
    pub client_id: String,
    pub code_verifier: Option<String>,
    pub code: String,
    pub data: Data,
}

impl AppContext {
    pub fn new() -> Result<Self, String> {
        let mut cx = Self::default();
        
        let auth_file_contents = read_file(File::Auth)?;
        
        let auth_data: AuthFileContent = serde_json::from_str(auth_file_contents.as_str()).map_err(|x| x.to_string())?;
        
        cx.code = auth_data.code;
        cx.code_verifier = Some(auth_data.code_verifier);
        
        Ok(cx)
    }

    pub fn set_code_verifier(&mut self) {}
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            client_id: "d46aab9576a9435593e70791f3cf70d7".to_string(),
            code_verifier: None,
            code: String::new(),
            data: Default::default(),
        }
    }
}
