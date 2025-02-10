use crate::db::get_dynamodb_client;
use crate::errors::NoteError;
use crate::models::{NoteMessage, NoteRequest, NoteResponse};
use aws_sdk_dynamodb::types::AttributeValue;

pub async fn read_note(request: NoteRequest) -> Result<NoteResponse, NoteError> {
    let client = get_dynamodb_client()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to get DynamoDB client".to_string()))?;

    let id = request.id.ok_or(NoteError::InvalidRequest(
        "Missing id in read request".to_string(),
    ))?;

    let response = client
        .get_item()
        .table_name("notes")
        .key("id", AttributeValue::S(id.clone()))
        .send()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to fetch note from DynamoDB".to_string()))?;

    if let Some(item) = response.item {
        let content = item
            .get("content")
            .and_then(|val| val.as_s().ok())
            .map(|s| s.to_string());

        return Ok(NoteResponse {
            id: Some(id),
            message: NoteMessage::NoteFound,
            content,
        });
    }

    Err(NoteError::NoteNotFound)
}
