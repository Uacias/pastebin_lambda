use crate::{
    db::NOTES,
    errors::NoteError,
    models::{Note, NoteMessage, NoteRequest, NoteResponse},
};

pub async fn update_note(request: NoteRequest) -> Result<NoteResponse, NoteError> {
    let id = request.id.ok_or(NoteError::InvalidRequest(
        "Missing id in update request.".to_string(),
    ))?;

    let content = request.content.ok_or(NoteError::InvalidRequest(
        "Missing content in update request.".to_string(),
    ))?;

    let mut notes = NOTES.lock().map_err(|_| NoteError::Internal)?;

    if notes.contains_key(&id) {
        notes.insert(
            id.clone(),
            Note {
                id: id.clone(),
                content: content.clone(),
            },
        );
        return Ok(NoteResponse {
            message: NoteMessage::NoteUpdated,
            id: Some(id),
            content: Some(content),
        });
    }
    Err(NoteError::NoteNotFound)
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::{create::create_note, update::update_note},
        models::{NoteAction, NoteMessage, NoteRequest},
    };

    #[tokio::test]
    async fn test_update_note() {
        let create_request = NoteRequest {
            action: NoteAction::Create,
            id: None,
            content: Some("Old content".to_string()),
        };

        let create_response = create_note(create_request).await.unwrap();
        let update_request = NoteRequest {
            action: NoteAction::Update,
            id: create_response.id.clone(),
            content: Some("New content".to_string()),
        };

        let response = update_note(update_request).await.unwrap();
        assert_eq!(response.message, NoteMessage::NoteUpdated);
        assert_eq!(response.content, Some("New content".to_string()));
    }

    #[tokio::test]
    async fn test_update_non_existent_note() {
        let update_request = NoteRequest {
            action: NoteAction::Update,
            id: Some("non-existent-id".to_string()),
            content: Some("Updated content".to_string()),
        };

        let response = update_note(update_request).await;
        assert!(response.is_err());
    }
}
