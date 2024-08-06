use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GetSessionFile {
    pub file_id: i64,
    pub session_number: usize,
    pub root_folder_id:i64
}
