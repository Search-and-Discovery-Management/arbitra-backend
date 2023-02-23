use actix_web::{post, web::{self, Data}, HttpResponse, get, put, delete};
use serde_json::json;
use crate::{EClient, routes::{str_or_default_if_exists_in_vec, document_struct::*}};

/// Inserts a new document, with 3 dynamic modes: true, false, strict
#[post("/api/document")]
pub async fn add_data_to_index(data: web::Json<DocumentCreate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {  
    let dat = data.into_inner();
    
    let set_dynamic_mode = match dat.dynamic_mode{
        Some (x) => Some(str_or_default_if_exists_in_vec(&x, vec!["true".to_string(), "false".to_string(), "strict".to_string()], "strict")),
        None => None
    };

    elasticsearch_client.insert_document(&dat.index, dat.data, set_dynamic_mode).await
}

/// Returns a list of documents from index
#[post("/api/search")]
pub async fn search_in_index(data: web::Json<DocumentSearch>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.search_index(&data.index, data.search_term.clone(), data.search_in.clone(), data.return_fields.clone(), data.from, data.count).await
}

#[get("/api/search/{index}")]
pub async fn get_search_in_index(data: web::Path<GetDocumentSearchIndex>, query: web::Query<GetDocumentSearchQuery>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.search_index(&data.index, query.search_term.clone(), query.search_in.clone(), query.return_fields.clone(), query.from, query.count).await
}

/// Returns a specific document
#[get("/api/document/{index}/{document_id}")]
pub async fn get_document_by_id(data: web::Path<DocById>, return_fields: web::Query<ReturnFields>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = data.into_inner();
    let fields_to_return = return_fields.into_inner().return_fields;

    elasticsearch_client.get_document(dat.index, dat.document_id, fields_to_return).await
}

/// Updates document on index
#[put("/api/document")]
pub async fn update_data_on_index(data: web::Json<DocumentUpdate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    // Update document on index

    // doc is required for updating index, read:
    // https://stackoverflow.com/questions/57564374/elasticsearch-update-gives-unknown-field-error
    let doc = json!({
        "doc": data.data.clone()
    });

    elasticsearch_client.update_document(&data.index, &data.document_id, doc).await
}

/// Deletes document in index
#[delete("/api/document/{index}/{document_id}")]
pub async fn delete_data_on_index(document_to_delete: web::Path<DocumentDelete>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = document_to_delete.into_inner();
    elasticsearch_client.delete_document(&dat.index, &dat.document_id).await
}