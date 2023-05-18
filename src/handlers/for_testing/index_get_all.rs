use actix_web::{web::Data, HttpResponse};
use ijson::IValue;
use serde_json::json;

use crate::{actions::EClient, handlers::{errors::ErrorTypes, libs::is_server_up}};

pub async fn get_indexes_list_debug(client: Data::<EClient>) -> HttpResponse {
    println!("Route: Debug get all indexes");

    if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    let indexes: Vec<IValue> = client.cat_get_index(Some("*".to_string())).await.unwrap().json::<Vec<IValue>>().await.unwrap();

    HttpResponse::Ok().json(json!(indexes))
}