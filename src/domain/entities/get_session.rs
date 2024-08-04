use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetSession {
    pub session_number: usize,
}
