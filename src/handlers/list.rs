use crate::db::get_dynamodb_client;
use crate::errors::NoteError;
use crate::models::Note;

pub async fn list_notes() -> Result<Vec<Note>, NoteError> {
    let client = get_dynamodb_client()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to get DynamoDB client".to_string()))?;

    let result = client
        .scan()
        .table_name("notes")
        .send()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to fetch notes from DynamoDB".to_string()))?;

    let notes: Vec<Note> = match result.items {
        Some(items) => items
            .iter()
            .filter_map(|item| {
                let id = item.get("id").and_then(|val| val.as_s().ok()).cloned();
                let content = item.get("content").and_then(|val| val.as_s().ok()).cloned();

                match (id, content) {
                    (Some(id), Some(content)) => Some(Note { id, content }),
                    _ => None,
                }
            })
            .collect(),
        None => Vec::new(),
    };

    Ok(notes)
}
