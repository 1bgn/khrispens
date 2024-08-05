use serde::Serialize;
use crate::domain::models::session_bundle::SessionBundle;
use crate::domain::models::session_file::SessionFile;

#[derive(Serialize)]
pub enum WebsocketEvent {
    FileEvent,
    SessionEvent

}