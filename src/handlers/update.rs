use crate::db::get_dynamodb_client;
use crate::errors::NoteError;
use crate::models::{NoteMessage, NoteRequest, NoteResponse};
use aws_sdk_dynamodb::types::AttributeValue;

pub async fn update_note(request: NoteRequest) -> Result<NoteResponse, NoteError> {
    let client = get_dynamodb_client()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to get DynamoDB client".to_string()))?;

    let id = request.id.ok_or(NoteError::InvalidRequest(
        "Missing id in update request".to_string(),
    ))?;
    let content = request.content.ok_or(NoteError::InvalidRequest(
        "Missing content in update request.".to_string(),
    ))?;

    let existing_note = client
        .get_item()
        .table_name("notes")
        .key("id", AttributeValue::S(id.clone()))
        .send()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to fetch note from DynamoDB".to_string()))?;

    if existing_note.item.is_none() {
        return Err(NoteError::NoteNotFound);
    }

    client
        .update_item()
        .table_name("notes")
        .key("id", AttributeValue::S(id.clone()))
        .update_expression("SET content = :content")
        .expression_attribute_values(":content", AttributeValue::S(content.clone()))
        .send()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to update note in DynamoDB".to_string()))?;

    Ok(NoteResponse {
        id: Some(id),
        message: NoteMessage::NoteUpdated,
        content: Some(content),
    })
}
