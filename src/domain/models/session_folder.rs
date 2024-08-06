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
    pub included_folder_ids:Vec<i64>,
    pub system_path:String,
    pub parent_id:i64

}
impl SessionFolder{
    pub fn new( parent_id:i64,to:String,create_session_folder:&CreateSessionFolder)->Self{
        Self{
            folder_name:create_session_folder.folder_name.clone(),
            id:rand::thread_rng().gen_range(0..10000000),
            create_at:Utc::now(),
            update_at:None,
            parent_id,
            system_path:format!("{}{}/",to,create_session_folder.folder_name),
            // included_folders:vec![],
            included_file_ids:vec![],
            included_folder_ids:vec![],
        }
    }
    pub fn root_new( to:String,create_session_folder:&CreateSessionFolder)->Self{
        Self{
            folder_name:create_session_folder.folder_name.clone(),
            id:1,
            parent_id:0,
            create_at:Utc::now(),
            update_at:None,

            system_path:format!("{}{}/",to,create_session_folder.folder_name),
            // included_folders:vec![],
            included_file_ids:vec![],
            included_folder_ids:vec![],
        }
    }
    pub fn add_file(&mut self,session_file: &SessionFile){
        self.included_file_ids.push(session_file.id);
        self.update_at = Some(Utc::now());
    }
    pub fn add_folder(&mut self,session_folder: &SessionFolder){
        self.included_folder_ids.push(session_folder.id);
        self.update_at = Some(Utc::now());
    }
}