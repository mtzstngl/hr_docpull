use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Document {
    #[serde(rename(deserialize = "ATT_FOLDER"))]
    pub folder: String,
    #[serde(rename(deserialize = "ATT_DOMAIN"))]
    pub domain: String,
    #[serde(rename(deserialize = "ATT_BOOKMARK"))]
    pub bookmark: String,
    #[serde(rename(deserialize = "ATT_NAME"))]
    pub name: String,
    #[serde(rename(deserialize = "FILE_INDEX"))]
    pub file_index: String,
    #[serde(rename(deserialize = "ATT_FOLDER_DESCRIPTION"))]
    pub folder_description: String,
    #[serde(rename(deserialize = "ATT_DOC_DATE"))]
    pub date: String,
    #[serde(rename(deserialize = "ATT_NOTIZ"))]
    pub note: String,
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    pub id: String,
    pub path: String,
    pub description: String,
    #[serde(rename(deserialize = "customFolder"))]
    pub custom_folder: bool,
    #[serde(rename(deserialize = "documentCount"))]
    pub document_count: u32,
    #[serde(rename(deserialize = "unreadDocumentCount"))]
    pub unread_document_count: u32,
    pub folders: Vec<Folder>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub id: String,
    pub description: String,
    #[serde(rename(deserialize = "type"))]
    pub type_number: i32,
    pub visible: bool,
    pub editable: bool,
    pub length: u32,
}

#[derive(Debug, Deserialize)]
pub struct HrDocumentBox {
    pub success: bool,
    #[serde(rename(deserialize = "totalResultCount"))]
    pub total_result_count: u32,
    #[serde(rename(deserialize = "totalCount"))]
    pub total_count: u32,
    #[serde(rename(deserialize = "unreadCount"))]
    pub unread_count: u32,
    pub offset: u32,
    #[serde(rename(deserialize = "metaData"))]
    pub metadata: Vec<Metadata>,
    pub documents: Vec<Document>,
    pub folders: Option<Vec<Folder>>,
}
