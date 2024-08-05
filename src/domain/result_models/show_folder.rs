use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::domain::models::session_file::SessionFile;
use crate::domain::models::session_folder::SessionFolder;

#[derive(Serialize)]
pub struct ShowFolder{
    pub id: i64,
    pub folder_name:String,
    pub create_at:DateTime<Utc>,
    pub update_at:Option<DateTime<Utc>>,
    pub included_files:Vec<SessionFile>,
    pub included_folders:Vec<SessionFolder>,
    pub system_path:String,
}
impl ShowFolder{
    pub fn new(session_folder: &SessionFolder,included_files:Vec<SessionFile>,included_folders:Vec<SessionFolder>)->ShowFolder{
        return  ShowFolder{
            id:session_folder.id,
            folder_name:session_folder.folder_name.clone(),
            create_at:session_folder.create_at,
            update_at:session_folder.update_at,
            included_files,
            included_folders,
            system_path:session_folder.system_path.clone()
        };
    }
}