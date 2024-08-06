use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::ws::Message;
use axum::Json;
use chrono::format::Item;
use dashmap::DashMap;
use serde::Serialize;
use crate::domain::models::session_bundle::SessionBundle;
use tokio::sync::{broadcast::{self, Receiver, Sender}, Mutex, MutexGuard};

#[derive(Clone)]
pub struct AppState {
    pub sessions: Arc<DashMap<usize,SessionBundle>>,
    pub broadcast_txs: Arc<DashMap<usize, Sender<Message>>>,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            sessions:Arc::new(DashMap::new()),
            broadcast_txs: Arc::new(DashMap::new())
        }
    }
    pub fn move_of(& self, index: usize) -> Sender<Message>{
        let (tx, _) = broadcast::channel(32);
        self.broadcast_txs.entry(index).or_insert( tx);
        return self.broadcast_txs.get(&index).unwrap().clone();
    }
    pub fn remove_of(&mut self, index: usize) {
        self.broadcast_txs.remove(&index);
    }
}