use actix_web::{HttpResponse, web::{self, Data}};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{actions::EClientTesting, handlers::libs::{check_server_up_exists_app_index, index_name_builder}};

#[derive(Deserialize)]
pub struct CreateBulkDocuments{
    pub app_id: String,
    pub index: String,
    pub data: Vec<Value>,
    pub dynamic_mode: String
}

/// TODO: Turn into an actually usable function
/// 
/// TODO: Mappings -> Always enable dynamic mode (and set it back to how it was previously?)
/// 
/// ? Return the errors? or only the count of errors?
/// 
/// Bulk Document Create Input, Only allows input into an existing index
pub async fn testing_create_bulk_documents(data: web::Json<CreateBulkDocuments>, client: Data::<EClientTesting>) -> HttpResponse {

    // create_or_exists_index(Some(data.app_id.to_string()), &data.index, None, None, &client).await;

    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let name = index_name_builder(&data.app_id, &data.index);

    let resp = client.bulk_create_documents(&name, &data.data).await.unwrap();

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