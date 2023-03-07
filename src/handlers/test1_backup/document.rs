use actix_web::{web::{self, Data}, HttpResponse};
use reqwest::StatusCode;

use crate::{models::{helpers::index_exists_check_or_down, ErrorTypes}, routes::str_or_default_if_exists_in_vec};

use super::document_struct::DocumentCreate;
use crate::actions::*;
use serde_json::{json};

/// Inserts a new document, with 3 dynamic modes: true, false, strict
#[allow(unused_must_use)] // index mapping update for dynamic mode
pub async fn create_document(data: web::Json<DocumentCreate>, elasticsearch_client: Data::<EClientTesting>) -> HttpResponse {  
    let dat = data.into_inner();
    
    let dynamic_mode = match dat.dynamic_mode{
        Some (x) => Some(str_or_default_if_exists_in_vec(&x, vec!["true".to_string(), "false".to_string(), "strict".to_string()], "strict")),
        None => None
    };

    let index = &dat.index;
    let data = dat.data;
    
    match index_exists_check_or_down(&elasticsearch_client.elastic, index).await{
        Ok(()) => (),
        Err(x) => return x
    };

    match dynamic_mode {
        Some(mode) => {
            let set_dynamic = json!({
                "dynamic": mode
            });
            elasticsearch_client.update_index_mappings(index, set_dynamic).await;
        },
        None => (),
    }

    let resp_err = elasticsearch_client.insert_document(index, data).await;

    let resp = match resp_err{
        Ok(x) => x,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": ErrorTypes::ServerDown.to_string()}))
    };

    let status_code = resp.status_code();

    if !status_code.is_success() {
        let error = match status_code{
            StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };
        return HttpResponse::build(status_code).json(json!({"error": error}));
    }

    let set_dynamic = json!({
        "dynamic": "strict"
    });
        
    elasticsearch_client.update_index_mappings(index, set_dynamic).await;

    HttpResponse::build(status_code).finish()
}