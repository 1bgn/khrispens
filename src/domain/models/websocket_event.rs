use serde::Serialize;

#[derive(Serialize)]
pub enum WebsocketEvent {
    FileEvent,
    FileDeletedEvent,
    FileRenamedEvent,
    SessionEvent,
    FolderCreateEvent,
    FolderDeletedEvent,
    FolderRenamedEvent
}