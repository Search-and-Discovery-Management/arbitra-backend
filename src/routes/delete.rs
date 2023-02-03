use serde_json::Value;
use actix_web::{delete, web::{self, Data}, Responder, Result};
use crate::EClient;


/*

JSON Data Format For Delete:
    {
        index: index_name
        document_id: id
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/
 
/// Deletes document in index
/// 
/// index: index_name
/// document_id: Document ID
#[delete("/api/delete_document")]
pub async fn delete_data_on_index(document_to_delete: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    let index = document_to_delete.get("index").map(|val| val.as_str().unwrap());

    let doc_id = document_to_delete.get("document_id").map(|val| val.as_str().unwrap());
    
    let resp = elasticsearch_client.delete_document(index, doc_id).await;

    Ok(web::Json(resp))
}