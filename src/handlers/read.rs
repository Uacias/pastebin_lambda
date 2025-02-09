use crate::{
    db::NOTES,
    errors::NoteError,
    models::{NoteMessage, NoteRequest, NoteResponse},
};

pub async fn read_note(request: NoteRequest) -> Result<NoteResponse, NoteError> {
    let id = request.id.ok_or(NoteError::InvalidRequest(
        "Missing id in read note request".to_string(),
    ))?;

    let notes = NOTES.lock().map_err(|_| NoteError::Internal)?;

    if let Some(note) = notes.get(&id) {
        return Ok(NoteResponse {
            message: NoteMessage::NoteFound,
            id: Some(id),
            content: Some(note.content.clone()),
        });
    }
    Err(NoteError::NoteNotFound)
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::{create::create_note, read::read_note},
        models::{NoteAction, NoteMessage, NoteRequest},
    };

    #[tokio::test]
    async fn test_read_note() {
        let create_request = NoteRequest {
            action: NoteAction::Create,
            id: None,
            content: Some("Test note".to_string()),
        };
        let create_response = create_note(create_request).await.unwrap();

        let read_request = NoteRequest {
            action: NoteAction::Read,
            id: Some(create_response.id.unwrap()),
            content: None,
        };
        let read_response = read_note(read_request).await.unwrap();
        assert_eq!(read_response.message, NoteMessage::NoteFound);
        assert_eq!(read_response.content, Some("Test note".to_string()));
    }

    #[tokio::test]
    async fn test_read_non_existent_note() {
        let read_request = NoteRequest {
            action: NoteAction::Read,
            id: Some("non-existent-id".to_string()),
            content: None,
        };

        let response = read_note(read_request).await;
        assert!(response.is_err());
    }
}
