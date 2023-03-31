use std::time::Duration;

use actix_web::{HttpResponse, web::{self, Data}};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

use crate::{actions::EClientTesting, handlers::libs::create_or_exists_index};

#[derive(Deserialize)]
pub struct CreateBulkDocuments{
    pub index: String,
    pub data: Vec<Value>
}

/// TODO: Turn into an actually usable function
pub async fn testing_create_bulk_documents(client: Data::<EClientTesting>, data: web::Json<CreateBulkDocuments>) -> HttpResponse {

    create_or_exists_index(None, &data.index, None, None, &client).await;

    tokio::time::sleep(Duration::from_secs(5)).await;

    let resp = client.bulk_create_documents(&data.index, &data.data).await.unwrap();

    let status = resp.status_code();
    let json: Value = resp.json::<Value>().await.unwrap();

    if json["errors"].as_bool().unwrap() {
        let failed: Vec<&Value> = json["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| !v["error"].is_null())
            .collect();

        println!("Errors whilst indexing. Failures: {}", failed.len());
        return HttpResponse::build(StatusCode::MULTI_STATUS).json(serde_json::json!({
            "error_count": failed.len(),
            "errors": failed
        })
        )
    }
    
    HttpResponse::build(status).finish()
}