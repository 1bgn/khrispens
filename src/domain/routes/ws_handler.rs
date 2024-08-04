use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use axum::{Error, extract::ws::{WebSocket, Message}};
use axum::body::Body;
use axum::extract::{ConnectInfo, Query, State, WebSocketUpgrade};
use axum::response::Response;
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{broadcast, Mutex};
use crate::domain::entities::get_session::GetSession;
// use crate::ws_state::WsState;
use tracing::{error, info};
use crate::app_state::AppState;
use crate::domain::models::session_bundle::SessionBundle;
use crate::ws_state::WsState;

pub async fn ws_handler(Query(s): Query<GetSession>, ws: WebSocketUpgrade, State(mut app_state): State<Arc< Mutex<State<WsState>>>>, ConnectInfo(addr): ConnectInfo<SocketAddr>) ->Response {
    info!("SESSION NUMBER #{}", s.session_number);
    println!("SESSION NUMBER #{}", s.session_number);
    let mut app_state= app_state.lock().await;
    let (tx, _) = broadcast::channel(32);
    app_state.broadcast_txs.entry(s.session_number).or_insert_with(|| Arc::new(Mutex::new(tx)));
    let broadcast_tx = app_state.broadcast_txs.get(&s.session_number).unwrap().clone();
    println!("session {:?}",app_state.broadcast_txs);
    return  ws.on_upgrade(move |socket| handle_socket(socket,s.session_number,  broadcast_tx));
}
async fn handle_socket(mut socket: WebSocket, session_number:usize,broadcast_tx :Arc<Mutex<Sender<Message>>>) {
    // let broadcast_tx = app_state.move_of(session_number);

    let (ws_tx, ws_rx) = socket.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));
    // let state_clone =Arc::clone();

    {
        let broadcast_rx = broadcast_tx.lock().await.subscribe();
        tokio::spawn(async move {
            recv_broadcast(&ws_tx, broadcast_rx).await;
        });
    }
    recv_from_client(ws_rx,&broadcast_tx).await;
}

async fn recv_from_client(
    mut client_rx: SplitStream<WebSocket>,
    broadcast_tx: &Arc<Mutex<Sender<Message>>>,
) {
    while let Some(Ok(msg)) = client_rx.next().await {

        if matches!(msg, Message::Close(_)) {
            println!("DISCOnneCteD");
            return;
        }
        if broadcast_tx.lock().await.send(msg).is_err() {
            println!("Failed to broadcast a message");
        }
    }
}
async fn recv_broadcast(
    client_tx: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    mut broadcast_rx: Receiver<Message>,
) {
    while let Ok(msg) = broadcast_rx.recv().await {
        if client_tx.lock().await.send(msg).await.is_err() {
            return; // disconnected.
        }
    }
}
