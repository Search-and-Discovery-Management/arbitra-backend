use actix_web::{put, web::{self, Data}, HttpResponse};
use serde::{Deserialize};
use serde_json::{Value, json};
use crate::{EClient};

#[derive(Deserialize)]
pub struct DocumentUpdate {
    index: String,
    document_id: String,
    data: Value
}

#[derive(Deserialize)]
pub struct IndexMappingUpdate {
    index: String,
    mappings: Value
}

/// Updates document on index
/// 
/// ```
/// Input Example:
///     json!({
///         "index": "index_name", 
///         "document_id": "document_id"
///         "data": {
///             "name": "username_test",
///             "password": "test_password",
///             ...
///         }
///     })
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Success
/// 400: Bad Request
/// 404: Not Found
/// ```
/// 
/// Does not return body if success
/// 
/// Example Error Body Example:
/// ```
/// {
///     "message": "not found"
/// }
/// ```
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

/// Updates the mappings of an index
/// 
/// ```
/// Input Example:
///     json!({
///         "index": "index_name", 
///         "mapping": {
///             "dynamic": true
///         }
///     })
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Success
/// 400: Bad Request
/// 404: Not Found
/// ```
/// 
/// Does not return body
#[put("/api/mappings")]
pub async fn index_mapping_update(data: web::Json<IndexMappingUpdate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    // Updates the mappings of an index, including its datatypes
    elasticsearch_client.update_index_mappings(&data.index, data.mappings.clone()).await
}