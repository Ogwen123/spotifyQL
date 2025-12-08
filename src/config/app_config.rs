pub struct AppContext {
    client_id: String,
    code_verifier: Option<String>
}

impl AppContext {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_code_verifier(&mut self) {
        
    }
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            client_id: "d46aab9576a9435593e70791f3cf70d7".to_string(),
            code_verifier: None
        }
    }
}