use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SessionFile {
    pub fileName: String,
    pub localFilepath: Option<String>,
    pub downloadUrl: Option<String>,
    pub createAt: Option<String>,
    pub updateAt: Option<String>,
    pub length: i64,
    pub id: i64,
}
