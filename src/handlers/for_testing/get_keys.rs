use std::{collections::HashMap};

use actix_web::{web::Data, HttpResponse};
use serde_json::Value;

use crate::{actions::EClientTesting};




pub async fn test_get_keys(client: Data<EClientTesting>) -> HttpResponse{
    // let name = &index_name_builder(&data.app_id, &data.index);
    let maps = client.get_index_mappings("airplanes_v3").await.unwrap();
    let keys: Value = maps.json::<Value>().await.unwrap();
    let val1: HashMap<String, Value> = serde_json::from_value(keys["airplanes_v3"]["mappings"]["properties"].clone()).unwrap();
    for (i, val) in val1 {
        println!("string: {i}, val: {:#?}", val);
    };
    HttpResponse::Ok().finish()
}

// pub async fn get_mapping_keys(index: &str, client: Data<EClientTesting>) -> Vec<String>{
//     let maps = client.get_index_mappings(index).await.unwrap();
//     let resp_json: Value = maps.json::<Value>().await.unwrap();
//     let val: HashMap<String, Value> = serde_json::from_value(resp_json[index]["mappings"]["properties"].clone()).unwrap();

//     val.iter().map(|(x, _)| x.to_string()).collect()
// }