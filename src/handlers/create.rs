use crate::db::get_dynamodb_client;
use crate::errors::NoteError;
use crate::models::{NoteMessage, NoteRequest, NoteResponse};
use aws_sdk_dynamodb::types::AttributeValue;
use uuid::Uuid;

pub async fn create_note(request: NoteRequest) -> Result<NoteResponse, NoteError> {
    let client = get_dynamodb_client()
        .await
        .map_err(|_| NoteError::DynamoDB("Failed to get DynamoDB client".to_string()))?;

    let id = Uuid::new_v4().to_string();
    let content = request.content.unwrap_or_default();

    tracing::info!(
        "📝 Creating note in DynamoDB: id={} content={}",
        id,
        content
    );

    let result = client
        .put_item()
        .table_name("notes")
        .item("id", AttributeValue::S(id.clone()))
        .item("content", AttributeValue::S(content.clone()))
        .send()
        .await;

    match result {
        Ok(_) => tracing::info!("✅ Note added DynamoDB!"),
        Err(e) => tracing::error!("❌ Error adding note to  DynamoDB: {:?}", e),
    }

    Ok(NoteResponse {
        id: Some(id),
        message: NoteMessage::NoteCreated,
        content: Some(content),
    })
}
