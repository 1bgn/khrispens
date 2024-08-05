use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetSessionFolder{
    pub system_path:String,
    pub session_number:usize
}