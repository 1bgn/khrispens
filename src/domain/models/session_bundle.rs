use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::Serialize;

use super::session_file::SessionFile;
use std::sync::Arc;
use axum::extract::ws::Message;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};
use crate::domain::entities::create_session_folder::CreateSessionFolder;
use crate::domain::models::session_cell::SessionCell;
use crate::domain::models::session_folder::SessionFolder;

#[derive(Clone, Serialize)]
pub struct SessionBundle {
    pub session_number: usize,
    pub id: i64,
    pub start_point:String,
    pub create_at: DateTime<Utc>,
    pub update_at: Option<DateTime<Utc>>,
    pub files: Vec<SessionFile>,
    pub included_folders: HashMap<String, SessionFolder>,
}
impl SessionBundle {
    pub fn new(session_number: usize) -> Self {
        Self {
            included_folders: HashMap::from([(String::from("/"), SessionFolder::new("".to_string(),&CreateSessionFolder {session_number:session_number, folder_name: String::from(""), system_path: String::from("/") }))]),
            start_point:String::from("/"),
            create_at: Utc::now(),
            update_at: None,
            session_number,
            files: vec![],
            id: rand::thread_rng().gen_range(0..10000000),
        }
    }
    pub fn add_folder(&mut self,session_folder: SessionFolder){
        &self.included_folders.insert(session_folder.system_path.clone(),session_folder);
    }
}
