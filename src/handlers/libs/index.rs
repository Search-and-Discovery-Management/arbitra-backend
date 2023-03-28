// use std::collections::HashMap;

use reqwest::StatusCode;
use serde_json::{json};

use crate::{actions::EClientTesting, handlers::errors::ErrorTypes};

use super::application::get_app_indexes_list;


/// Inserts a new index into an application
pub async fn create_or_exists_index(app_id: Option<String>, index: &str, shards: Option<usize>, replicas: Option<usize>, client: &EClientTesting) -> StatusCode {

    // Check if index exists
    let index_name = match app_id {
        Some(x) => index_name_builder(&x, index).to_string(),
        None => index.to_string()
    };

    let exists = client.check_index(&index_name).await.unwrap();

    // If exists, return conflict
    if exists.status_code().is_success() {
        return StatusCode::CONFLICT;
    }

    // If not found, create a new index
    if exists.status_code() == StatusCode::NOT_FOUND {
        let body = 
            json!(
                {
                    "mappings": { 	
                        "dynamic":"true"
                    },
                    "settings": {
                        "index.number_of_shards": shards.unwrap_or(3),
                        "index.number_of_replicas": replicas.unwrap_or(0),
                    }
                }
            );
        let resp = client.create_index(&index_name, &body).await.unwrap();
        let status = resp.status_code();
        // println!("{:#?}", resp.json::<Value>().await.unwrap());
        if status == StatusCode::OK {
            return StatusCode::CREATED;
        }
    }

    exists.status_code()
}

pub fn index_name_builder(app_id: &str, index_name: &str) -> String{
    format!("{}.{}", app_id.trim().to_ascii_lowercase(), index_name.trim().to_ascii_lowercase())
}

// pub async fn get_mapping_keys(index: &str, client: &EClientTesting) -> Vec<String>{
//     let maps = client.get_index_mappings(index).await.unwrap();
//     let resp_json: Value = maps.json::<Value>().await.unwrap();
//     println!("{:#?}", resp_json);
//     let val: Result<Vec<(String, Value)>, serde_json::Error> = serde_json::from_value(resp_json[index]["mappings"]["properties"].clone());

//     match val {
//         Ok(fields) => fields.iter().map(|(x, _)| x.to_string()).collect(),
//         Err(_) => Vec::new()
//     }
// }

pub async fn index_exists(app_id: &str, index_name: &str, client: &EClientTesting) -> Result<(usize, Vec<String>), (StatusCode, ErrorTypes, Vec<String>)> {
    // let resp = client.get_document(APPLICATION_LIST_NAME, app_id, Some("indexes".to_string())).await;

    // let resp_json = match resp {
    //     Ok(x) => {
    //         x.json::<Value>().await.unwrap()
    //     },
    //     Err(_) => {
    //         return None
    //     }
    // };

    // let list: Vec<String> = match resp_json.get("indexes") {
    //     Some(x) => serde_json::from_value(x.clone()).unwrap(),
    //     None => Vec::new()
    // };

    let list = match get_app_indexes_list(app_id, client).await {
        Ok(x) => x,
        Err((status, error)) => return Err((status, error, Vec::new()))
    };

    match list.iter().position(|x| x.eq(index_name)) {
        Some(x) => return Ok((x, list)),
        None => return Err((StatusCode::NOT_FOUND, ErrorTypes::IndexNotFound(index_name.to_string()), list))
    }
}