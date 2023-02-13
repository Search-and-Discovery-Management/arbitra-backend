use serde_json::{Value};
use actix_web::{delete, web::{self, Data},  HttpResponse};
use crate::{EClient, routes::required_check_string};


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
#[delete("/api/document")]
pub async fn delete_data_on_index(document_to_delete: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> HttpResponse {

    let idx = match required_check_string(document_to_delete.get("index"), "index"){
        Ok(x) => x,
        Err(x) => return x
    };

    let document_id = match required_check_string(document_to_delete.get("document_id"), "document id"){
        Ok(x) => x,
        Err(x) => return x
    };
    
    elasticsearch_client.delete_document(&idx, &document_id).await
}