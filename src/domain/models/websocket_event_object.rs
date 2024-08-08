use serde::Serialize;

use crate::domain::models::websocket_event::WebsocketEvent;

#[derive(Serialize)]
pub struct WebsocketEventObject<T:Serialize>{
    pub websocket_event_type: WebsocketEvent,
    pub data:T,
    pub folder:i64
}