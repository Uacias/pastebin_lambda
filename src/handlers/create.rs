use uuid::Uuid;

use crate::{
    db::NOTES,
    errors::NoteError,
    models::{Note, NoteMessage, NoteRequest, NoteResponse},
};

pub async fn create_note(request: NoteRequest) -> Result<NoteResponse, NoteError> {
    let id = Uuid::new_v4().to_string();
    let content = request.content.unwrap_or_default();
    let mut notes = NOTES.lock().map_err(|_| NoteError::Internal)?;
    notes.insert(
        id.clone(),
        Note {
            id: id.clone(),
            content: content.clone(),
        },
    );

    Ok(NoteResponse {
        id: Some(id),
        message: NoteMessage::NoteCreated,
        content: Some(content),
    })
}

#[cfg(test)]
mod tests {
    use crate::models::{NoteAction, NoteMessage};

    use super::*;
    #[tokio::test]
    async fn test_create_note() {
        let request = NoteRequest {
            action: NoteAction::Create,
            id: None,
            content: Some("Test note".to_string()),
        };
        let repsonse = create_note(request).await.unwrap();
        assert_eq!(repsonse.message, NoteMessage::NoteCreated);
    }
}
