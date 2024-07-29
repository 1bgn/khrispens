use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetSession {
    pub id: i64,
}
