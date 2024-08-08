use chrono::{DateTime, Utc};

pub trait  SessionCell{
    fn id(&self)->i64;
    fn system_path(&self)->String;
    fn create_at(&self)->DateTime<Utc>;
    fn update_at(&self)->Option<DateTime<Utc>>;
    fn name(&self)->String;

}