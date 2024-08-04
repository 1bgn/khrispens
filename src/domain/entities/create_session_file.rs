use std::usize;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateSessionFile {
    pub session_number: usize,
    pub filename: String,
}
