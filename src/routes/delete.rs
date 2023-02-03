use serde_json::Value;
use actix_web::{delete, web::{self, Data}, Responder, Result};
use crate::EClient;


/*

JSON Data Format For Delete(Might change):
    {
        index: index_name
        document_id: id
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/

#[delete("/api/delete_data")]
pub async fn delete_data_on_index(search_term: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    // Deletes the data inside the index

    // Ok(web::Json(users.user_list.clone()))

    // Index: index_name
    // Document_id: id

    // TODO: Check if either index or doc_id is empty, early return
    let index = search_term.get("index").unwrap().to_string();
    let doc_id = search_term.get("document_id").unwrap().to_string();
    
    let status = elasticsearch_client.delete_document(&index, &doc_id).await;

    println!("STATUS: {:#?}", status);

    Ok(web::Json(search_term.clone()))
}