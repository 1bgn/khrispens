use chrono::{DateTime, Utc};
use rand::Rng;
use serde::de::Unexpected::Str;
use serde::Serialize;
use crate::domain::entities::create_session_folder::CreateSessionFolder;
use crate::domain::models::session_cell::SessionCell;
use crate::domain::models::session_file::SessionFile;

#[derive(Serialize,Clone,Debug)]
pub struct SessionFolder{
    pub id: i64,
    pub folder_name:String,
    pub create_at:DateTime<Utc>,
    pub update_at:Option<DateTime<Utc>>,
    pub included_file_ids:Vec<i64>,
    pub system_path:String,

}
impl SessionFolder{
    pub fn new( to:String,create_session_folder:&CreateSessionFolder)->Self{
        Self{
            folder_name:create_session_folder.folder_name.clone(),
            id:rand::thread_rng().gen_range(0..10000000),
            create_at:Utc::now(),
            update_at:None,
            system_path:format!("{}{}/",to,create_session_folder.folder_name),
            // included_folders:vec![],
            included_file_ids:vec![]
        }
    }
    pub fn add_file(&mut self,session_file: &SessionFile){
        self.included_file_ids.push(session_file.id);
        self.update_at = Some(Utc::now());
    }
}