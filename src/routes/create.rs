use crate::EClient;
use actix_web::{post, web::{self, Data}, Responder, Result};
use reqwest::StatusCode;
use serde_json::{Value, json};

// Temporary hardcode to add test data

#[post("/api/hardcoded_data_add")]
pub async fn hardcoded_data_for_testing(elasticsearch_client: Data::<EClient>) -> impl Responder{

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
        elasticsearch_client.insert_document(INDEX, data).await;
    }
    // println!("{:#?}", y);

    // println!("{:#?}", elasticsearch_client.insert_document(INDEX, y).await);
    // let successful = response.status_code().is_success();
    "Hello {app_name}!" // temp: Avoid error
}

/*
JSON Data Format For Creating new document:
    {
        index: index_name
        data: {
            name
            password
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/

#[post("/api/create_document")]
pub async fn add_data_to_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    let idx = match data.get("index"){
            Some(val) => val.as_str().unwrap(),
            None => return Ok(web::Json(json!({
                "error_message": "Index not supplied",
                "status_code": StatusCode::BAD_REQUEST.as_u16()
            })))
        };

    let to_input = match data.get("data") {
            Some(val) => val.clone(),
            None => return Ok(web::Json(json!({
                "error_message": "Data not supplied",
                "status_code": StatusCode::BAD_REQUEST.as_u16()
            }))),
    };
    // println!("{:#?}", ind);
    // println!("\n\n");
    // println!("{:#?}", to_input);
    let status = elasticsearch_client.insert_document(idx, to_input).await;

    let x = json!({
        "status": status.as_str()
    });
    Ok(web::Json(x))
}

/*
JSON Data Format For Creating new Index:
    {
        index: index_name
    }
*/

// Only creates index

#[post("/api/create_index")]
pub async fn create_new_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {
    let index_to_create = data.get("index");

    let status = elasticsearch_client.create_index(index_to_create.unwrap().as_str().unwrap()).await;

    println!("{:#?}", status);
    Ok(web::Json(json!({
        "status_code": status.as_u16()
    })))
}