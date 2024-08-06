use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::ws::Message;
use axum::Json;
use chrono::format::Item;
use serde::Serialize;
use crate::domain::models::session_bundle::SessionBundle;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};

#[derive(Clone)]
pub struct AppState {
    pub sessions: Vec<SessionBundle>,
    pub broadcast_txs: HashMap<usize, Arc<Sender<Message>>>,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            sessions:vec![],
            broadcast_txs: HashMap::new()
        }
    }
    pub fn move_of(&mut self, index: usize) -> Arc<Sender<Message>>{
        let (tx, _) = broadcast::channel(32);

        self.broadcast_txs.entry(index).or_insert( Arc::new(tx));
        return self.broadcast_txs.get(&index).unwrap().clone();
    }
    pub fn remove_of(&mut self, index: usize) {
        self.broadcast_txs.remove(&index);
    }
}