use actix_web::{delete, web::{self, Data}, HttpResponse};
use serde::{Deserialize};
use crate::{EClient};
 
#[derive(Deserialize)]
pub struct DocumentDelete {
    index: String,
    document_id: String
}

#[derive(Deserialize)]
pub struct IndexDelete {
    index: String
}

/// Deletes document in index
/// ```
/// index: index_name
/// document_id: document_id
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Deleted successfully
/// 404: Not Found
/// ```
/// 
/// Returns body:
/// ```
/// {
///     "message": "error_or_success"
/// }
/// ```
#[delete("/api/document/{index}/{document_id}")]
pub async fn delete_data_on_index(document_to_delete: web::Path<DocumentDelete>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = document_to_delete.into_inner();
    elasticsearch_client.delete_document(&dat.index, &dat.document_id).await
}

/// Deletes an index
#[delete("/api/index/{index}")]
pub async fn delete_index(index_to_delete: web::Path<IndexDelete>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = index_to_delete.into_inner();
    elasticsearch_client.delete_index(&dat.index).await
}