use actix_web::{put, web::{self, Data}, Responder, Result};
use serde_json::Value;
use crate::EClient;

/*
JSON Data Format For Update:
    {
        "index": index_name,
        "document_id": document_id,
        "data": {
            "doc": {
                "name": "",
                "password": ""
                ...
            }
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");

    doc is required for updating, read:
    https://stackoverflow.com/questions/57564374/elasticsearch-update-gives-unknown-field-error
    
*/

#[put("/api/update_document")]
pub async fn update_data_on_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {
    // Update data on index

    let index = data.get("index").map(|val| val.as_str().unwrap());
    let doc_id = data.get("document_id").map(|val| val.as_str().unwrap());
    let to_input = data.get("data").map(|val| val.clone());

    let resp = elasticsearch_client.update_document(index, doc_id, to_input).await;

    Ok(web::Json(resp))
}