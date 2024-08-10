use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub struct CreateSessionFolder{
    pub root_folder_id:i64,
    pub session_number: String,
    pub folder_name:String,
}