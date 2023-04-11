use actix_web::{web::{self, Data}, HttpResponse};
use client::EClientTesting;
use serde_json::{Value, json};

use crate::{handlers::{libs::{check_server_up_exists_app_index, index_name_builder}, structs::index_struct::RequiredIndex, structs::document_struct::BulkFailures}, actions::client};

// TODO: Preprocess input data to extract the id
pub async fn update_bulk_documents(app_index: web::Path<RequiredIndex>, data: web::Json<Vec<Value>>, client: Data<EClientTesting>) -> HttpResponse {
    let idx = app_index.index.clone().trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let name = index_name_builder(&app_index.app_id, &idx);

    let resp = client.bulk_update_documents(&name, "_id",&data).await.unwrap();

    let status = resp.status_code();
    let json: Value = resp.json::<Value>().await.unwrap();

    let mut failures: Vec<BulkFailures> = vec![];
    if json["errors"].as_bool().unwrap() {
        for (loc, val) in json["items"].as_array().unwrap().iter().enumerate(){
            if !val["update"]["error"].is_null(){
                failures.push(
                    BulkFailures {
                        document_number: loc,
                        error: val["update"]["error"]["reason"].as_str().unwrap().to_string(),
                        status: val["update"]["status"].as_i64().unwrap()
                    }
                );
            }
        }
    }
    
    HttpResponse::build(status).json(serde_json::json!({
        "error_count": failures.len(),
        "has_errors": json["errors"].as_bool().unwrap(),
        "errors": failures
    }))
}