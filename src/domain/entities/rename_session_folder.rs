use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameSessionFolder {
    pub root_folder_id:i64,
    pub session_number:String,
    pub new_name:String
}