use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub username: String,
}
