use crate::utils::file::{delete_file, File};

pub fn logout() -> Result<(), String> {
    // delete auth file
    delete_file(File::Auth)
}
