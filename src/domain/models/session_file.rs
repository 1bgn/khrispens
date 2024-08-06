use chrono::{DateTime, Utc};
use rand::Rng;
use serde::Serialize;

use crate::domain::entities::create_session_file::CreateSessionFile;
use crate::domain::models::session_cell::SessionCell;

#[derive(Clone, Serialize)]
pub struct SessionFile {
    pub file_name: String,
    pub local_filepath: Option<String>,
    pub system_path:String,
    pub download_url: Option<String>,
    pub create_at: DateTime<Utc>,
    pub update_at: Option<DateTime<Utc>>,
    pub length: Option<usize>,
    pub id: i64,
    pub session_file_state: SessionFileState,
}
impl SessionCell for SessionFile {
    fn id(&self) -> i64 {
        self.id
    }

    fn system_path(&self) -> String {
        self.system_path.clone()
    }

    fn create_at(&self) -> DateTime<Utc> {
        self.create_at
    }

    fn update_at(&self) -> Option<DateTime<Utc>> {
        self.update_at
    }

    fn name(&self) -> String {
        self.file_name.clone()
    }
}
impl SessionFile {
    pub fn new( create_session_file:&CreateSessionFile) -> Self {
        Self {
            system_path:String::new(),
            file_name: create_session_file.filename.clone(),
            local_filepath: None,
            download_url: None,
            create_at: Utc::now(),
            update_at: None,
            length: None,
            session_file_state: SessionFileState::Uploading,
            id: rand::thread_rng().gen_range(0..10000000),
        }
    }
    pub fn upload(&mut self, local_path: String, download_url: String, length: usize) ->&mut  SessionFile{
        self.local_filepath = Some(local_path);
        self.download_url = Some(download_url);
        self.session_file_state = SessionFileState::Uploaded;
        self.length = Some(length);
        self.update_at = Some(Utc::now());
        return  self;
    }
}
#[derive(Clone, Serialize)]
pub enum SessionFileState {
    Uploading,
    Uploaded,
}
