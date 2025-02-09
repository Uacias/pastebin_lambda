use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoteError {
    #[error("Invalid action {0}")]
    InvalidAction(String),

    #[error("Note nod found")]
    NoteNotFound,

    #[error("Failed to parse request {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Internal server error")]
    Internal,

    #[error("Invalid request {0}")]
    InvalidRequest(String),
}
