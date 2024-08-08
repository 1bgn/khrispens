use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rand::Rng;
use serde::Serialize;

use crate::domain::entities::create_session_folder::CreateSessionFolder;
use crate::domain::models::session_folder::SessionFolder;

use super::session_file::SessionFile;

#[derive(Clone, Serialize)]
pub struct SessionBundle {
    pub session_number: usize,
    pub id: i64,
    pub start_point:i64,
    pub create_at: DateTime<Utc>,
    pub update_at: Option<DateTime<Utc>>,
    pub files: HashMap<i64,SessionFile>,
    pub included_folders: HashMap<i64, SessionFolder>,
}
impl SessionBundle {
    pub fn new(session_number: usize) -> Self {
        Self {
            included_folders: HashMap::from([(1, SessionFolder::root_new("".to_string(),&CreateSessionFolder { root_folder_id: 0, session_number:session_number, folder_name:String::from("root") }))]),
            start_point:1,
            create_at: Utc::now(),
            update_at: None,
            session_number,
            files: HashMap::new(),
            id: rand::thread_rng().gen_range(0..10000000),
        }
    }
    pub fn add_folder(&mut self,session_folder: SessionFolder){
        &self.included_folders.insert(session_folder.id,session_folder);
    }

}
