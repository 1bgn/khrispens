use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ErrorMessage {
    pub message: String,
}
impl ErrorMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
