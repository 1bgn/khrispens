use std::sync::Arc;
use axum::extract::ws::Message;
use serde::Serialize;
use crate::domain::models::session_bundle::SessionBundle;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};

#[derive(Clone)]
pub struct WsState {
    pub broadcast_tx: Arc<Mutex<Sender<Message>>>,
}
