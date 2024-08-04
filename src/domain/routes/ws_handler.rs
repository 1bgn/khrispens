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

pub async fn ws_handler(Query(s): Query<GetSession>, ws: WebSocketUpgrade, State(mut app_state): State<Arc<Mutex<State<WsState>>>>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Response {
    info!("SESSION NUMBER #{}", s.session_number);
    println!("SESSION NUMBER #{}", s.session_number);
    let mut app_state = app_state.lock().await;

    let broadcast_tx = app_state.move_of(s.session_number);
    println!("socket_sessions {:?}", app_state.broadcast_txs);
    return ws.on_upgrade(move |socket| handle_socket(socket, s.session_number, broadcast_tx));
}
async fn handle_socket(socket: WebSocket, session_number: usize, broadcast_tx: Arc<Sender<Message>>) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    while let Some(Ok(msg)) = ws_rx.next().await {
        match msg {
            Message::Text(text) => {
                break;
            }
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(_) => {
                return;
            }
        }
    }

    let mut guard = broadcast_tx.clone();
    let mut rx = guard.subscribe();
    guard.send(Message::Text("hi".to_string()));


    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });
    let mut guard = broadcast_tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = ws_rx.next().await {
            // Add username before message.
            let _ = guard.send(Message::Text("teafe".to_string()));
        }
    });
    println!("TEST");
}
// async fn handle_socket(socket:WebSocket, session_number:usize, broadcast_tx: Arc<Mutex<Sender<Message>>>){
//     let(mut sender,mut receiver)= socket.split();
//     while let Some (Ok(message)) = receiver.next().await{
//
//     }
//     let mut rx = broadcast_tx.lock().await.subscribe();
//     let mut send_task = tokio::spawn(async move {
//         while let Ok(msg) = rx.recv().await {
//             // In any websocket error, break loop.
//             if sender.send(Message::Text("t".to_string())).await.is_err() {
//                 break;
//             }
//         }
//     });
//     let mut recv_task = tokio::spawn(async move {
//         while let Some(Ok(Message::Text(text))) = receiver.next().await {
//             // Add username before message.
//             let _ = broadcast_tx.lock().send(format!("'ferve"));
//         }
//     });
// }
// async fn handle_socket(mut socket: WebSocket, session_number:usize,broadcast_tx :Arc<Mutex<Sender<Message>>>) {
//     let (ws_tx, ws_rx) = socket.split();
//     let ws_tx = Arc::new(Mutex::new(ws_tx));
//     // let state_clone =Arc::clone();
//
//     {
//         let guard =  broadcast_tx.lock().await;
//         let broadcast_rx = guard.subscribe();
//         println!("CONNECTED {}",Arc::strong_count(&broadcast_tx));
//         tokio::spawn(async move {
//             recv_broadcast(&ws_tx, broadcast_rx).await;
//         });
//     }
//
//
//     recv_from_client(ws_rx,&broadcast_tx,).await;
// }

// async fn recv_from_client(
//     mut client_rx: SplitStream<WebSocket>,
//    broadcast_tx: &Arc<Mutex<Sender<Message>>>,
// ) {
//
//     while let Some(Ok(msg)) = client_rx.next().await {
//
//         if matches!(msg, Message::Close(_)) {
//             if Arc::strong_count(broadcast_tx)==2{
//                 println!("Все отключились");
//             }
//             return;
//         }
//         if broadcast_tx.lock().await.send(msg).is_err() {
//             println!("Failed to broadcast a message");
//         }
//     }
//
//
// }
// async fn recv_broadcast(
//     client_tx: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
//     mut broadcast_rx: Receiver<Message>,
// ) {
//     while let Ok(msg) = broadcast_rx.recv().await {
//         if client_tx.lock().await.send(msg).await.is_err() {
//             return; // disconnected.
//         }
//     }
// }
