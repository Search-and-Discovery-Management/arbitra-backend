use actix_web::{web::{self, Data}, HttpResponse};
use client::EClient;
use elasticsearch::{BulkOperations, BulkOperation};
use serde_json::{Value, json};

use crate::{handlers::{libs::{check_server_up_exists_app_index, index_name_builder}, structs::index_struct::RequiredIndex, structs::document_struct::{BulkFailures, RequiredRequest}}, actions::client, AppConfig};

// TODO: Preprocess input data to extract the id
// TODO: Refactor
pub async fn update_bulk_documents(app_index: web::Path<RequiredIndex>, data: web::Json<Vec<RequiredRequest>>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse {
    let idx = app_index.index.clone().trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client, &app_config).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let name = index_name_builder(&app_index.app_id, &idx);

    let mut update_datas: Vec<Value> = vec!{};
    let mut update_ids: Vec<String> = vec!{};
    let mut update_shards: Vec<usize> = vec![];
    for i in data.iter() {
        update_ids.push(i.document_id.to_owned());
        update_datas.push(i.data.to_owned());
        update_shards.push(i.document_id.split('.').last().unwrap().parse::<usize>().unwrap())
    }

    let resp = client.bulk_update_documents(&name, &update_datas, &update_ids, &update_shards).await.unwrap();

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

/// Untested separated BulkOperations
pub async fn test_update(app_index: web::Path<RequiredIndex>, data: web::Json<Vec<RequiredRequest>>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse {
    let idx = app_index.index.clone().trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client, &app_config).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let name = index_name_builder(&app_index.app_id, &idx);

    let mut body: BulkOperations = BulkOperations::new();
    for i in data.iter() {
        let update_shard: usize = i.document_id.split('.').last().unwrap().parse::<usize>().unwrap();
        body.push(BulkOperation::update(&i.document_id, &i.data).index(format!("{}.{}", name, update_shard))).unwrap();
    }
    let resp = client.bulk(body).await.unwrap();

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