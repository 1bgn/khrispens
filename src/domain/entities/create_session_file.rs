use std::usize;

use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub struct CreateSessionFile {
    pub session_number: usize,
    pub filename: String,
    pub system_path:String
}
