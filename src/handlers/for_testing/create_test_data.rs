use actix_web::{HttpResponse, web::{Data, self}};
use futures::future::join_all;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{actions::EClientTesting, handlers::libs::{create_or_exists_index, index_name_builder, get_app_indexes_list}, APPLICATION_LIST_NAME};

#[derive(Deserialize)]
pub struct TestDataInsert {
    pub app_id: String,
    pub index: Option<String>,
    pub shards: Option<usize>,
    pub replicas: Option<usize>,
    pub link: Option<String>
}


#[allow(unused_must_use)]
pub async fn test_data(data: web::Json<TestDataInsert>, client: Data::<EClientTesting>) -> HttpResponse{
    // const INDEX: &str = "airplanes_v3";

    let idx = data.index.clone().unwrap_or("airplanes_v3".to_string());
        
    let index_exists = create_or_exists_index(Some(data.app_id.clone()), &idx, data.shards, data.replicas, &client).await;

    if !index_exists.is_success() && !index_exists.eq(&StatusCode::CONFLICT){
        return HttpResponse::build(index_exists).finish();
    }

    let name = index_name_builder(&data.app_id, &idx);

    // No question mark for await, https://github.com/actix/actix-web/wiki/FAQ
    // let resp = reqwest::Client::new()
    //     .get("https://raw.githubusercontent.com/algolia/datasets/master/airports/airports.json")
    //     .send()
    //     .await;

    let resp = get_app_indexes_list(&data.app_id, &client).await;
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
            let _ = client.update_document(APPLICATION_LIST_NAME, &data.app_id, &body).await;
        },
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }
    
    // If dynamic mode has value, set to whatever is inputted
    let body = json!({
        "dynamic": true
    });
    let _ = client.update_index_mappings(&name, &body).await;

    let resp = reqwest::Client::new()
        .get(&data.link.clone().unwrap_or("https://raw.githubusercontent.com/algolia/datasets/master/airports/airports.json".to_string()))
        .send()
        .await;

    let x = resp.unwrap();

    let y = x.json::<Vec<Value>>().await.unwrap();
    let mut vals = Vec::new();
    for dat in &y {
        // client.insert_document(&name, &dat).await;
        vals.push(client.insert_document(&name, dat));
    }
    let x = join_all(vals).await;
    println!("{:#?}", x[0]);

    HttpResponse::Ok().finish()
}