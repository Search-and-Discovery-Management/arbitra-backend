use actix_web::{web::{self, Data}, HttpResponse};
use serde_json::json;
use crate::{EClient, routes_backup::{str_or_default_if_exists_in_vec, document_struct::*}};

/// Inserts a new document, with 3 dynamic modes: true, false, strict
pub async fn create_document(data: web::Json<DocumentCreate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {  
    let dat = data.into_inner();
    
    let set_dynamic_mode = match dat.dynamic_mode{
        Some (x) => Some(str_or_default_if_exists_in_vec(&x, vec!["true".to_string(), "false".to_string(), "strict".to_string()], "strict")),
        None => None
    };

    elasticsearch_client.insert_document(&dat.index, dat.data, set_dynamic_mode).await
}

/// Returns a list of documents from index, post method
pub async fn post_search(data: web::Json<DocumentSearch>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.search_index(&data.index, data.search_term.clone(), data.search_in.clone(), data.return_fields.clone(), data.from, data.count).await
}

/// Returns a list of documents from index
pub async fn search(data: web::Path<GetDocumentSearchIndex>, query: web::Query<GetDocumentSearchQuery>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.search_index(&data.index, query.search_term.clone(), query.search_in.clone(), query.return_fields.clone(), query.from, query.count).await
}

/// Returns a specific document
pub async fn get_document(data: web::Path<DocById>, return_fields: web::Query<ReturnFields>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = data.into_inner();
    let fields_to_return = return_fields.into_inner().return_fields;

    elasticsearch_client.get_document(dat.index, dat.document_id, fields_to_return).await
}

/// Updates document on index
pub async fn update_document(data: web::Json<DocumentUpdate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    // Update document on index

    // doc is required for updating index, read:
    // https://stackoverflow.com/questions/57564374/elasticsearch-update-gives-unknown-field-error
    let doc = json!({
        "doc": data.data.clone()
    });

    elasticsearch_client.update_document(&data.index, &data.document_id, doc).await
}

/// Deletes document in index
pub async fn delete_document(document_to_delete: web::Path<DocumentDelete>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = document_to_delete.into_inner();
    elasticsearch_client.delete_document(&dat.index, &dat.document_id).await
}