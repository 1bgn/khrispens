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

#[derive(Clone, Serialize)]
pub struct SessionBundle {
    pub session_number: usize,
    pub id: i64,
    pub create_at: DateTime<Utc>,
    pub update_at: Option<DateTime<Utc>>,
    pub files: Vec<SessionFile>,

}
impl SessionBundle {
    pub fn new(session_number: usize) -> Self {

        Self {

            create_at: Utc::now(),
            update_at: None,
            session_number,
            files: vec![],
            id: rand::thread_rng().gen_range(0..10000000),
        }
    }
}
