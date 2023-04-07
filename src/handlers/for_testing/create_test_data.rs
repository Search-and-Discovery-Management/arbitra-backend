use actix_web::{HttpResponse, web::{Data, self}};
// use futures::future::join_all;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{actions::EClientTesting, handlers::{libs::{index_name_builder, get_app_indexes_list}, applications_struct::RequiredAppID}, APPLICATION_LIST_NAME};

#[derive(Deserialize)]
pub struct TestDataInsert {
    pub index: Option<String>,
    pub shards: Option<usize>,
    pub replicas: Option<usize>,
    pub link: Option<String>
}

pub async fn test_data(app: web::Path<RequiredAppID>, data: web::Json<TestDataInsert>, client: Data::<EClientTesting>) -> HttpResponse{
    // const INDEX: &str = "airplanes_v3";

    let idx = data.index.clone().unwrap_or("airplanes_v3".to_string());
        
    // let index_exists = create_or_exists_index(Some(data.app_id.clone()), &idx, data.shards, data.replicas, &client).await;

    // if !index_exists.is_success() && !index_exists.eq(&StatusCode::CONFLICT){
    //     return HttpResponse::build(index_exists).finish();
    // }

    // tokio::time::sleep(Duration::from_secs(5));

    let name = index_name_builder(&app.app_id, &idx);

    // No question mark for await, https://github.com/actix/actix-web/wiki/FAQ
    // let resp = reqwest::Client::new()
    //     .get("https://raw.githubusercontent.com/algolia/datasets/master/airports/airports.json")
    //     .send()
    //     .await;

    let resp = get_app_indexes_list(&app.app_id, &client).await;
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
            let _ = client.update_document(APPLICATION_LIST_NAME, &app.app_id, &body).await;
        },
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }
    
    // If dynamic mode has value, set to whatever is inputted
    // let body = json!({
    //     "dynamic": true
    // });
    // let _ = client.update_index_mappings(&name, &body).await;

    let resp = reqwest::Client::new()
        .get(&data.link.clone().unwrap_or("https://raw.githubusercontent.com/algolia/datasets/master/airports/airports.json".to_string()))
        .send()
        .await;

    let x = resp.unwrap();

    let y = x.json::<Vec<Value>>().await.unwrap();
    // let mut vals = Vec::new();
    // for dat in &y {
    //     // client.insert_document(&name, &dat).await;
    //     vals.push(client.insert_document(&name, dat));
    // }
    // let x = join_all(vals).await;
    // println!("{:#?}", x[0]);

    let resp = client.bulk_create_documents(&name, &y).await.unwrap();
    // tokio::time::sleep(Duration::from_secs(5));

    let status = resp.status_code();
    let json: Value = resp.json::<Value>().await.unwrap();

    if json["errors"].as_bool().unwrap() {
        let failed: Vec<&Value> = json["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| !v["error"].is_null())
            .collect();

        println!("Errors whilst indexing. Failures: {}", failed.len());
        return HttpResponse::build(StatusCode::MULTI_STATUS).json(serde_json::json!({
            "error_count": failed.len(),
            "errors": failed
        })
        )
    }

    // println!("{:#?}", json);

    HttpResponse::build(status).finish()
}

