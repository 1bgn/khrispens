use chrono::{DateTime, Utc};
use rand::Rng;
use serde::Serialize;

use crate::domain::entities::create_session_file::CreateSessionFile;

#[derive(Clone, Serialize)]
pub struct SessionFile {
    pub file_name: String,
    pub local_filepath: Option<String>,
    pub download_url: Option<String>,
    pub create_at: DateTime<Utc>,
    pub update_at: Option<DateTime<Utc>>,
    pub length: Option<usize>,
    pub id: i64,
    pub session_file_state: SessionFileState,
}
impl SessionFile {
    pub fn new(create_session_file: CreateSessionFile) -> Self {
        Self {
            file_name: create_session_file.filename,
            local_filepath: None,
            download_url: None,
            create_at: Utc::now(),
            update_at: None,
            length: None,
            session_file_state: SessionFileState::Uploading,
            id: rand::thread_rng().gen_range(0..10000000),
        }
    }
    pub fn upload(&mut self, local_path: String, download_url: String, length: usize) {
        self.local_filepath = Some(local_path);
        self.download_url = Some(download_url);
        self.session_file_state = SessionFileState::Uploaded;
        self.length = Some(length);
        self.update_at = Some(Utc::now());
    }
}
#[derive(Clone, Serialize)]
pub enum SessionFileState {
    Uploading,
    Uploaded,
}
