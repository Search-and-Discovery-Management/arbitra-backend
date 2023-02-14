use actix_web::{delete, web::{self, Data}, HttpResponse};
use serde::{Deserialize};
use crate::{EClient};
 
#[derive(Deserialize)]
pub struct DocumentDelete {
    index: String,
    document_id: String
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

// #[delete("/api/document")]
// pub async fn delete_data_on_index(document_to_delete: web::Json<DocumentDelete>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
//     elasticsearch_client.delete_document(&document_to_delete.index, &document_to_delete.document_id).await
// }