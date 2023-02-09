use actix_web::{put, web::{self, Data}, HttpResponse};
use serde_json::{Value};
use crate::{EClient, routes::{required_check_string, required_check_value}};

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

#[put("/api/document")]
pub async fn update_data_on_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    // Update document on index

    let idx = match required_check_string(data.get("index"), "index"){
        Ok(x) => x,
        Err(x) => return x
    };

    let document_id = match required_check_string(data.get("document_id"), "document id"){
        Ok(x) => x,
        Err(x) => return x
    };

    let to_update = match required_check_value(data.get("data"), "data"){
        Ok(x) => x,
        Err(x) => return x
    };
    
    // let index = match data.get("index") {
    //     Some(val) => val.as_str().unwrap(),
    //     None => 
    //         return HttpResponse::build(StatusCode::BAD_REQUEST).json(
    //             json!({
    //             "message": "Index not supplied"
    //         }))
    // };

    // let doc_id = match data.get("document_id"){
    //     Some(val) => val.as_str().unwrap(),
    //     None => 
    //         return HttpResponse::build(StatusCode::BAD_REQUEST).json(
    //             json!({
    //             "message": "Document ID not supplied"
    //         }))
    // };

    // let to_update = match data.get("data"){
    //     Some(val) => val.clone(),
    //     None => 
    //         return HttpResponse::build(StatusCode::BAD_REQUEST).json(
    //             json!({
    //             "message": "Data not supplied"
    //         }))
    // };

    elasticsearch_client.update_document(&idx, &document_id, to_update).await

    // HttpResponse::build(resp).finish()
}

#[put("/api/mappings")]
pub async fn index_mapping_update(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    // Update document on index
    // let index = match data.get("index") {
    //     Some(val) => val.as_str().unwrap(),
    //     None => 
    //         return HttpResponse::build(StatusCode::BAD_REQUEST).json(
    //             json!({
    //             "message": "Index not supplied"
    //         }))
    // };

    // let to_update = match data.get("mapping"){
    //     Some(val) => val.clone(),
    //     None => 
    //         return HttpResponse::build(StatusCode::BAD_REQUEST).json(
    //             json!({
    //             "message": "Mappings not supplied"
    //         }))
    // };

    let idx = match required_check_string(data.get("index"), "index"){
        Ok(x) => x,
        Err(x) => return x
    };

    let to_update = match required_check_value(data.get("mapping"), "data"){
        Ok(x) => x,
        Err(x) => return x
    };

    elasticsearch_client.update_index_mappings(&idx, to_update).await
}