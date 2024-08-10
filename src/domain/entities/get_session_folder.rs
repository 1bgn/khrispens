use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetSessionFolder{
    pub root_folder_id:i64,
    pub session_number:String
}