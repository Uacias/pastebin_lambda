use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NoteAction {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize)]
pub struct NoteRequest {
    pub action: NoteAction,
    pub id: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NoteMessage {
    NoteCreated,
    NoteFound,
    NoteUpdated,
    NoteDeleted,
    NoteNotFound,
    InvalidRequest,
    InternalError,
}

#[derive(Serialize, Deserialize)]
pub struct NoteResponse {
    pub message: NoteMessage,
    pub id: Option<String>,
    pub content: Option<String>,
}

pub struct Note {
    pub id: String,
    pub content: String,
}
