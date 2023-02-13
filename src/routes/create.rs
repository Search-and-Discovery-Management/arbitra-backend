use crate::{EClient, routes::{required_check_value, required_check_string, optional_check_string, str_or_default_if_exists_in_vec}};
use actix_web::{post, web::{self, Data}, HttpResponse};
use serde_json::{Value};

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

    println!("{:#?}", resp);
    println!("Test");

    let x = resp.unwrap();

    let y = x.json::<Vec<Value>>().await.unwrap();
    for data in y {
        elasticsearch_client.insert_document(INDEX, data, None).await;
    }

    HttpResponse::Ok().finish()
}

/// Inserts a new document, with 3 dynamic modes: true, false, strict
/// 
/// "true" -> allow creation of new fields
/// 
/// "false" -> does not allow creation of new fields, only inserts new entry to existing fields with the rest lost
/// 
/// "strict" -> does not insert if either partial or has new fields
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

#[post("/api/document")]
pub async fn add_data_to_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> HttpResponse {

    let idx = match required_check_string(data.get("index"), "index"){
        Ok(x) => x,
        Err(x) => return x
    };

    let to_input = match required_check_value(data.get("data"), "data"){
        Ok(x) => x,
        Err(x) => return x
    };

    let set_dynamic_mode = match optional_check_string(data.get("dynamic_mode")){
        Some (x) => Some(str_or_default_if_exists_in_vec(&x, vec!["true".to_string(), "false".to_string(), "strict".to_string()], "strict")),
        None => None
    };

    elasticsearch_client.insert_document(&idx, to_input, set_dynamic_mode).await
}

/*
JSON Data Format For Creating new Index:
    {
        "index": index_name
    }
*/

// Creates a new dynamic index

#[post("/api/index")]
pub async fn create_new_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> HttpResponse {

    let idx = match required_check_string(data.get("index"), "index"){
        Ok(x) => x,
        Err(x) => return x
    };

    elasticsearch_client.create_index(&idx).await

    // HttpResponse::build(status_code).finish()
}