use std::net::SocketAddr;
use std::ops::{ControlFlow, Deref, DerefMut};
use std::sync::{Arc, };

use axum::body::Body;
use axum::extract::{ConnectInfo, Query, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::Json;
use axum::response::Response;
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{broadcast, Mutex, MutexGuard};
use crate::domain::entities::get_session::GetSession;
// use crate::ws_state::WsState;
use tracing::{error, info};
use crate::app_state::AppState;
use crate::domain::models::session_bundle::SessionBundle;

pub async fn ws_handler(Query(s): Query<GetSession>, ws: WebSocketUpgrade, State(mut app_state): State<AppState>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Response {

    println!("SESSION NUMBER #{}", s.session_number);



    return ws.on_upgrade(move |socket| handle_socket(socket,addr, s.session_number, app_state));
}
async fn handle_socket(socket: WebSocket, who:SocketAddr, session_number: usize, mut app_state: AppState)  {

        let broadcast_tx = app_state.move_of(session_number);
        println!("socket_sessions {:?}", app_state.broadcast_txs);
        let (mut ws_tx, mut ws_rx) = socket.split();



        let mut sender = broadcast_tx.clone();
        let mut rx = sender.subscribe();
    //снимаем замок

    //
        let mut send_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if ws_tx.send(msg).await.is_err() {
                    println!("client  abruptly disconnected");
                    break;
                }else {
                }
            }
        });
        let mut recv_task = tokio::spawn(async move {

            while let Some(Ok(msg)) = ws_rx.next().await {
                if process_message(msg, who).is_break() {
                    break;
                }
            }
        });

        tokio::select! {
        rv_a = (&mut send_task) => {
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {

            send_task.abort();
        }
    }
        if broadcast_tx.receiver_count()==1{
            println!("Все отключились");
        }

        println!("Websocket context {who} destroyed, count: {}", broadcast_tx.receiver_count());

    app_state.remove_of(session_number);
}
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {

        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {

                println!(">>> {} somehow sent close message without CloseFrame",who);
            }
            return ControlFlow::Break(());
        }

        _ => {}
    }
    ControlFlow::Continue(())
}