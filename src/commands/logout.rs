use crate::utils::file::{File, delete_file};

pub fn logout() -> Result<(), String> {
    // delete auth file
    delete_file(File::Auth)
}
