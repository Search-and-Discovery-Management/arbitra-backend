use actix_web::{web::{Data}, HttpResponse};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{handlers::{libs::{is_server_up}, errors::ErrorTypes}, actions::EClient};

/// This deletes all indexes in elasticsearch, including "application_list"
pub async fn delete_everything(client: Data::<EClient>) -> HttpResponse {  
    println!("Route: Delete Everything");
    
    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    let indexes = client.cat_get_index(Some("*".to_string())).await.unwrap().json::<Vec<Value>>().await.unwrap();
    
    for i in indexes {
        let _ = client.delete_index(i["index"].as_str().unwrap()).await;
    }
    HttpResponse::Ok().finish()
}