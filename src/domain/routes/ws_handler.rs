use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Error, extract::ws::{WebSocket, Message}};
use axum::extract::{ConnectInfo, Query, State, WebSocketUpgrade};
use axum::response::Response;
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;
use crate::domain::entities::get_session::GetSession;
use crate::ws_state::WsState;
use tracing::{error, info};
#[tracing::instrument(skip(ws, state))]
pub async fn ws_handler(Query(s): Query<GetSession>, ws: WebSocketUpgrade, State(state): State<WsState>, ConnectInfo(addr): ConnectInfo<SocketAddr>) ->Response {
    info!("SESSION NUMBER #{}", s.session_number);
    println!("SESSION NUMBER #{}", s.session_number);
    return  ws.on_upgrade(move |socket| handle_socket(socket,state));
}
async fn handle_socket(mut socket: WebSocket, app_state: WsState) {

    let (ws_tx, ws_rx) = socket.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));
    // let state_clone =Arc::clone();
    let app = &app_state;
    {
        let broadcast_rx = app.broadcast_tx.lock().await.subscribe();
        tokio::spawn(async move {
            recv_broadcast(&ws_tx, broadcast_rx).await;
        });
    }
    recv_from_client(ws_rx,&app.broadcast_tx).await;
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
