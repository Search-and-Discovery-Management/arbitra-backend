
/*
JSON Data Format For Creating new Index:
    {
        "index": index_name
    }
*/

use actix_web::{web::Data, HttpResponse};
use serde_json::Value;

use crate::models_backup::EClient;

// Temporary hardcode to add test data
#[allow(unused_must_use)]
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