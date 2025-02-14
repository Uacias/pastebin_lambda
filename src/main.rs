use handlers::{
    create::create_note, delete::delete_note, list::list_notes, read::read_note,
    update::update_note,
};
use lambda_runtime::{service_fn, LambdaEvent};
use serde_json::json;
use std::error::Error;

pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;

use crate::models::{NoteAction, NoteRequest};

async fn handler(
    event: LambdaEvent<serde_json::Value>,
) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    tracing::info!("📝 Otrzymano żądanie: {:?}", event.payload);

    let http_method = event.payload.get("httpMethod").and_then(|m| m.as_str());

    if let Some("OPTIONS") = http_method {
        return Ok(json!({
            "statusCode": 200,
            "headers": cors_headers(),
            "body": ""
        }));
    }

    let request: NoteRequest = match http_method {
        Some("POST") | Some("PUT") | Some("DELETE") => {
            if let Some(body) = event.payload.get("body").and_then(|b| b.as_str()) {
                match serde_json::from_str(body) {
                    Ok(req) => req,
                    Err(_) => {
                        return Ok(json!({
                            "statusCode": 400,
                            "headers": cors_headers(),
                            "body": json!({ "error": "Bad Request: Invalid JSON" }).to_string()
                        }));
                    }
                }
            } else {
                return Ok(json!({
                    "statusCode": 400,
                    "headers": cors_headers(),
                    "body": json!({ "error": "Bad Request: Missing body" }).to_string()
                }));
            }
        }
        Some("GET") => {
            let empty_json = json!({});
            let query_params_json = event
                .payload
                .get("queryStringParameters")
                .unwrap_or(&empty_json);

            let id = query_params_json
                .get("id")
                .and_then(|i| i.as_str())
                .map(|s| s.to_string());

            if let Some(id) = id {
                NoteRequest {
                    action: NoteAction::Read,
                    id: Some(id),
                    content: None,
                }
            } else {
                // ✅ Pobieranie wszystkich notatek
                return match list_notes().await {
                    Ok(notes) => Ok(json!({
                        "statusCode": 200,
                        "headers": cors_headers(),
                        "body": json!(notes).to_string()
                    })),
                    Err(_) => Ok(json!({
                        "statusCode": 500,
                        "headers": cors_headers(),
                        "body": json!({ "error": "Internal Server Error" }).to_string()
                    })),
                };
            }
        }
        _ => {
            return Ok(json!({
                "statusCode": 405,
                "headers": cors_headers(),
                "body": json!({ "error": "Method Not Allowed" }).to_string()
            }));
        }
    };

    let response = match request.action {
        NoteAction::Create => create_note(request).await?,
        NoteAction::Read => read_note(request).await?,
        NoteAction::Update => update_note(request).await?,
        NoteAction::Delete => delete_note(request).await?,
    };

    Ok(json!({
        "statusCode": 200,
        "headers": cors_headers(),
        "isBase64Encoded": false,
        "body": json!(response).to_string()
    }))
}

// ✅ Upewnij się, że nagłówki CORS są poprawne
fn cors_headers() -> serde_json::Value {
    json!({
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods": "OPTIONS, GET, POST, PUT, DELETE",
        "Access-Control-Allow-Headers": "Authorization, Content-Type"
    })
}

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🔄 Lambda function starting...");

    let handler_fn = service_fn(handler);
    lambda_runtime::run(handler_fn).await?;

    Ok(())
}
