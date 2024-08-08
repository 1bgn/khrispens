use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameSessionFile {
    pub root_folder_id:i64,
    pub session_number:usize,
    pub file_id:i64,
    pub new_name:String
}