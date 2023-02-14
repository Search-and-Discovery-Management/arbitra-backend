use actix_web::{post, web::{self, Data}, HttpResponse};
use serde::{Deserialize};
use serde_json::{Value};
use crate::{EClient, routes::{str_or_default_if_exists_in_vec}};

#[derive(Deserialize)]
pub struct DocumentCreate{
    index: String,
    data: Value,
    dynamic_mode: Option<String>
}

#[derive(Deserialize)]
pub struct IndexCreate{
    index: String
}

/// Inserts a new document, with 3 dynamic modes: true, false, strict
/// 
/// "true" -> allow creation of new fields and partial inserts
/// 
/// "false" -> does not allow creation of new fields, only inserts new entry to existing fields with the rest lost
/// 
/// "strict" -> does not insert if it has new fields, allows partial inserts
/// 
/// Input example:
/// 
/// ```
/// json!({
///     "index": "test_index",
///     "dynamic_mode": true, // OPTIONAL
///     "data": {
///         "name": "name1",
///         "password": "password1",
///         "etc": "etc1"
///     }
/// )}
/// 
/// ```
/// 
/// Returns StatusCode: 
/// ```
/// 201: Data Created Successfully
/// 400: Bad Request 
/// 404: Not Found
/// ```
/// 
/// Does not return a body
#[post("/api/document")]
pub async fn add_data_to_index(data: web::Json<DocumentCreate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {  
    let dat = data.into_inner();
    
    let set_dynamic_mode = match dat.dynamic_mode{
        Some (x) => Some(str_or_default_if_exists_in_vec(&x, vec!["true".to_string(), "false".to_string(), "strict".to_string()], "strict")),
        None => None
    };

    elasticsearch_client.insert_document(&dat.index, dat.data, set_dynamic_mode).await
}

/*
JSON Data Format For Creating new Index:
    {
        "index": index_name
    }
*/

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

// Temporary hardcode to add test data
#[post("/api/hardcoded_data_add")]
pub async fn hardcoded_data_for_testing(elasticsearch_client: Data::<EClient>) -> HttpResponse{

    const INDEX: &str = "airplanes_v3";
        
    let index_exists = elasticsearch_client.create_index(INDEX).await;

    println!("{:#?}", index_exists);

    // No question mark for await, https://github.com/actix/actix-web/wiki/FAQ
    let resp = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/algolia/datasets/master/airports/airports.json")
        .send()
        .await;

    let x = resp.unwrap();

    let y = x.json::<Vec<Value>>().await.unwrap();
    for data in y {
        elasticsearch_client.insert_document(INDEX, data, None).await;
    }

    HttpResponse::Ok().finish()
}