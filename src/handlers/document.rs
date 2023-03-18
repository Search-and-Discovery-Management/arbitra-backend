use actix_web::{web::{self, Data}, HttpResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::{actions::EClientTesting, handlers::{libs::index_name_builder, errors::ErrorTypes}};

use super::document_struct::{DocumentCreate};

/// Document interfaces with index that is stored within the application id
/// Inserting a document with a new field syncs the fields with all other shards
/// 
/// All operations requires app_id and the index name
/// 

// Temp _ because models and routes having same name

pub async fn _create_document(data: web::Json<DocumentCreate>, client: Data::<EClientTesting>) -> HttpResponse {  
    // Creates a new document by getting application id, index name, check if document has new field, if yes, check dynamic mode
    // if true, update the entire index shards to accomodate the new field, then insert

    // TODO: Change ID to have appended shard number
    // Solutions: Either use fingerprint or use bulk api, the latter being most likely since it allows direct insert

    // Inserts document into index -> Checks if app has index
    // Checks if index exists
    // Insert

    // Used for Post: Document
    // #[derive(Deserialize)]
    // pub struct DocumentCreate{
    //     pub app_id: String,
    //     pub index: String,
    //     pub data: Value,
    //     pub dynamic_mode: Option<String>
    // }
    
    let name = index_name_builder(&data.app_id, &data.index);

    let dat = &data.data;
    let dynamic_mode = &data.dynamic_mode;
    
    // If dynamic mode has value, set to whatever is inputted
    if dynamic_mode.is_some() {
        let body = json!({
            "dynamic": dynamic_mode.as_ref().unwrap()
        });
        let _ = client.update_index_mappings(&name, &body).await;
    }
    
    let resp = client.insert_document(&name, dat).await.unwrap();

    // If dynamic mode doesnt have any value, change it back to strict mode
    if dynamic_mode.is_none() {
        let body = json!({
            "dynamic": "strict"
        });
        let _ = client.update_index_mappings(&name, &body).await;
    }
    
    let status = resp.status_code();

    match status {
        StatusCode::NOT_FOUND => HttpResponse::NotFound().json(json!({"error": ErrorTypes::IndexNotFound(name).to_string()})),
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().json(json!({"error": ErrorTypes::BadDataRequest.to_string()})),
        _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()})),
    }
}

pub async fn _get_document(client: Data::<EClientTesting>) -> HttpResponse {  
    // App id, index name, and document id
    // This will retrieve the shard number appended on the id

    todo!()
}

pub async fn _search(client: Data::<EClientTesting>) -> HttpResponse {  
    // Searches the whole index with wildcard to search all of the shards
    todo!()
}

pub async fn _update_document(client: Data::<EClientTesting>) -> HttpResponse {  
    // Updates the documents in shard

    todo!()
}

pub async fn _delete_document(client: Data::<EClientTesting>) -> HttpResponse {  
    // Deletes the document in shard

    todo!()
}