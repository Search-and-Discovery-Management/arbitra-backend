use actix_web::{HttpResponse, web::{Data, self}};
use reqwest::StatusCode;
// use futures::future::join_all;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{actions::EClient, handlers::{libs::{get_app_indexes_list, create_or_exists_index, bulk_create, is_server_up}, structs::applications_struct::RequiredAppID, errors::ErrorTypes}, AppConfig};

#[derive(Deserialize)]
pub struct TestDataInsert {
    pub index: Option<String>,
    pub shards: Option<usize>,
    pub replicas: Option<usize>,
    pub link: Option<String>
}

/// Inserts test data from a given URL
/// 
/// TODO: Error Handling
pub async fn test_data(app: web::Path<RequiredAppID>, optional_data: Option<web::Json<TestDataInsert>>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse{
    println!("Route: Create Test Data");

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    let data = if optional_data.is_some(){
        optional_data.as_deref().unwrap().to_owned()
    } else {
        &TestDataInsert{
            index: None,
            shards: None,
            replicas: None,
            link: None,
        }
    };
    let idx = data.index.clone().unwrap_or("airplanes_v3".to_string());

    let resp = get_app_indexes_list(&app.app_id, &client, &app_config).await;
    match resp {
        Ok(mut list) => {
            list.push(idx.clone());
            list.sort();
            list.dedup();
            let body = json!({
                "doc": {
                    "indexes": list
                }
            });
            let _ = client.update_document(&app_config.application_list_name, &app.app_id, &body).await;
            let _ = create_or_exists_index(Some(app.app_id.clone()), &idx, None, None, Some(10), &client, &app_config).await;
        },
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let resp = reqwest::Client::new()
        .get(&data.link.clone().unwrap_or("https://raw.githubusercontent.com/algolia/datasets/master/airports/airports.json".to_string()))
        .send()
        .await
        .unwrap();
    
    let data = resp.json::<Vec<Value>>().await.unwrap();

    println!("{:#?}", data[0]);

    bulk_create(&app.app_id, &idx, &data, &client, &app_config).await
}

