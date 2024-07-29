use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GetFile {
    pub id: i64,
    pub session_number: usize,
}
