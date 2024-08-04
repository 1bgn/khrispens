use std::collections::HashMap;
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
     pub broadcast_txs: HashMap<usize,Arc<Mutex<Sender<Message>>>>,
}
impl WsState{
    pub fn new()->Self{
      Self{
          broadcast_txs:HashMap::new()
      }
    }
   // pub  fn move_of(&mut  self, index:usize)-> Arc<Mutex<Sender<Message>>>{
   //      let (tx, _) = broadcast::channel(32);
   //     println!("sess: {:?}",self.broadcast_txs);
   //       ( self.broadcast_txs.entry(index)).or_insert_with(|| Arc::new(Mutex::new(tx)));
   //          return self.broadcast_txs.get(&index).unwrap().clone();
   //  }
}
