use actix_web::{web::{self, Data}, HttpResponse, post, get, put, delete};
use crate::{EClient, routes::{index_struct::*}};


/// Creates a new dynamic index
/// 
/// Input example:
/// ```
/// json!({
///     "index": "index_name"
/// )}
/// 
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 201: Created
/// 400: Bad Request
/// 409: Conflict (Already Exists)
/// ```
/// 
/// Does not return a body
#[post("/api/index")]
pub async fn create_new_index(data: web::Json<IndexCreate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.create_index(&data.index).await
}

/// Returns list of index if index is not provided, returns specified index if provided
/// 
/// Optional param: index
/// 
/// ```
/// Example:
///     127.0.0.1:8080/api/index?index=index-name
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Success
/// 404: Not Found
/// ```
/// 
/// Success Body Example:
/// ```
/// [
///     {
///         "docs.count": "0",
///         "docs.deleted": "0",
///         "health": "green",
///         "index": "test_index",
///         "pri": "3",
///         "pri.store.size": "675b",
///         "rep": "0",
///         "status": "open",
///         "store.size": "675b",
///         "uuid": "qyX3NoR8SXOPkA0EoiDWRg"
///     }
/// ]
/// ```
#[get("/api/index")]
async fn get_all_index(index: web::Query<OptionalIndex>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.get_index(index.into_inner().index).await
}

/// Returns the mappings of an index
#[get("/api/mappings/{index}")]
async fn get_mapping(index: web::Path<RequiredIndex>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.get_index_mappings(index.into_inner().index).await
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

/// Deletes an index
#[delete("/api/index/{index}")]
pub async fn delete_index(index_to_delete: web::Path<IndexDelete>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = index_to_delete.into_inner();
    elasticsearch_client.delete_index(&dat.index).await
}