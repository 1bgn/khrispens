use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct GetSession {
    pub session_number: String,
}
