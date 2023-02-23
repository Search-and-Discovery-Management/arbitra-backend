use actix_web::HttpResponse;
use elasticsearch::{indices::IndicesExistsParts, Elasticsearch};
use reqwest::StatusCode;
use serde_json::json;

use crate::models::ErrorTypes;

pub async fn server_down_check(server: &Elasticsearch) -> Result<(), HttpResponse> {
    let server = server
        .indices()
        .exists(IndicesExistsParts::Index(&["test"]))
        .send()
        .await;        
        
    match server {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(json!({"error": ErrorTypes::ServerDown.to_string()})))
        }
    };
}

pub async fn index_exists_check(server: &Elasticsearch, index: &str) -> Result<(), HttpResponse> {
    let index_check = server
        .indices()
        .exists(IndicesExistsParts::Index(&[index]))
        .send()
        .await       
        .unwrap();

    let status_code = index_check.status_code();

    if !status_code.is_success(){
        let error =  match status_code{
            StatusCode::NOT_FOUND => ErrorTypes::IndexNotFound(index.to_string()).to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };

        return Err(HttpResponse::build(status_code).json(json!({"error": error})));
    }

    Ok(())
}