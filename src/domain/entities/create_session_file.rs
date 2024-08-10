use std::usize;

use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub struct CreateSessionFile {
    pub session_number: String,
    pub filename: String,
    pub root_folder_id:i64
}
