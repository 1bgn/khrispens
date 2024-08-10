use std::sync::Arc;

use axum::extract::ws::Message;
use dashmap::DashMap;
use tokio::sync::broadcast::{self, Sender};

use crate::domain::models::session_bundle::SessionBundle;

#[derive(Clone)]
pub struct AppState {
    pub address: String,
    pub sessions: Arc<DashMap<String,SessionBundle>>,
    pub broadcast_txs: Arc<DashMap<String, Sender<Message>>>,
}
impl AppState {
    pub fn new(address:String) -> Self {
        Self {
            address,
            sessions:Arc::new(DashMap::new()),
            broadcast_txs: Arc::new(DashMap::new())
        }
    }
    pub fn move_of(& self, index: String) -> Sender<Message>{
        let (tx, _) = broadcast::channel(32);
        self.broadcast_txs.entry(index.clone()).or_insert( tx);
        return self.broadcast_txs.get(&index).unwrap().clone();
    }
    pub fn remove_of(&mut self, index: String) {
        self.broadcast_txs.remove(&index);
    }
}