use aws_sdk_dynamodb::types::AttributeValue;

use crate::db::get_dynamodb_client;
use crate::errors::NoteError;
use crate::models::{NoteMessage, NoteRequest, NoteResponse};

pub async fn delete_note(request: NoteRequest) -> Result<NoteResponse, NoteError> {
    let client = get_dynamodb_client()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to get DynamoDB client".to_string()))?;

    let id = request.id.ok_or(NoteError::InvalidRequest(
        "Missing id in delete request".to_string(),
    ))?;

    client
        .delete_item()
        .table_name("notes")
        .key("id", AttributeValue::S(id.clone()))
        .send()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to delete note from DynamoDB".to_string()))?;

    Ok(NoteResponse {
        id: Some(id),
        message: NoteMessage::NoteDeleted,
        content: None,
    })
}
