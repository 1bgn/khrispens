use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub struct CreateSessionFolder{
    pub folder_name:String,
    pub system_path:String,
    pub session_number: usize,
}