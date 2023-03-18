// A Collection of functions shared among handler functions
// TODO: Move functions to libs folder and separated into multiple parts

use elasticsearch::BulkCreateOperation;
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{actions::client::EClientTesting, APPLICATION_LIST_NAME};

enum BulkTypes {
    Create,
    Update,
    Delete
}




// UNUSED: Finish App First
// Outputs: app_id.index_name.shard_

// Lib function to create x amount of shards
// Returns nothing
// pub async fn create_shards(app_id: &str, index_name: &str, count: usize, shards: Option<usize>, replicas: Option<usize>, client: &EClientTesting){

//     let name = format!("{}.shard_", index_name_builder(app_id, index_name));

//     let body = 
//             json!(
//                 {
//                     "mappings": { 	
//                         "dynamic":"true"
//                     },
//                     "settings": {
//                         "index.number_of_shards": shards.unwrap_or(3),
//                         "index.number_of_replicas": replicas.unwrap_or(0),
//                     }
//                 }
//             );
//     for i in 0..count{
//         // let mut shard_name = name.clone();

//         let shard_name = format!("{name}{i}");
//         // shard_name.push_str(&i.to_string());
//         let _ = client.create_index(&shard_name, &body).await;
//     }
// }



// pub async fn generate_nanoid_with_shard(num_of_shards: usize) {


//     let random_id = nanoid::nanoid!(18);
    
// }

// pub async fn check_index_exists_and_resp(){

// }