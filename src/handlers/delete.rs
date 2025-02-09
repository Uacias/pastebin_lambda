use crate::{
    db::NOTES,
    errors::NoteError,
    models::{NoteMessage, NoteRequest, NoteResponse},
};

pub async fn delete_note(request: NoteRequest) -> Result<NoteResponse, NoteError> {
    let id = request.id.ok_or(NoteError::InvalidRequest(
        "Missing id in delete note request".to_string(),
    ))?;

    let mut notes = NOTES.lock().map_err(|_| NoteError::Internal)?;

    if notes.remove(&id).is_some() {
        return Ok(NoteResponse {
            message: NoteMessage::NoteDeleted,
            id: Some(id),
            content: None,
        });
    }
    Err(NoteError::NoteNotFound)
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::{create::create_note, delete::delete_note},
        models::{NoteAction, NoteMessage, NoteRequest},
    };

    #[tokio::test]
    async fn test_delete_note() {
        let create_request = NoteRequest {
            action: NoteAction::Create,
            id: None,
            content: Some("Delete me".to_string()),
        };

        let create_response = create_note(create_request).await.unwrap();
        let delete_request = NoteRequest {
            action: NoteAction::Delete,
            id: create_response.id.clone(),
            content: None,
        };

        let response = delete_note(delete_request).await.unwrap();
        assert_eq!(response.message, NoteMessage::NoteDeleted);
    }

    #[tokio::test]
    async fn test_delete_non_existent_note() {
        let delete_request = NoteRequest {
            action: NoteAction::Delete,
            id: Some("non-existent-id".to_string()),
            content: None,
        };

        let response = delete_note(delete_request).await;
        assert!(response.is_err());
    }
}
