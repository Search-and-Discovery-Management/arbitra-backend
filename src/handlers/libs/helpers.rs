use reqwest::StatusCode;

use crate::{actions::EClientTesting, handlers::{libs::index_exists, errors::ErrorTypes}};

/// Checks if the elastic server is up
pub async fn is_server_up(client: &EClientTesting) -> bool {
    client.check_index("1").await.is_ok()
}

/// Checks if 1. Server is up, 2. App and Index exists
pub async fn check_server_up_exists_app_index(app_id: &str, index: &str, client: &EClientTesting) -> Result<(), (StatusCode, ErrorTypes)>{

    // let (server_up, app_index_exists) = futures::join!(is_server_up(client), index_exists(app_id, index, &client));



    if is_server_up(client).await {
        match index_exists(app_id, index, client).await {
            Ok(_) => return Ok(()),
            Err((status, err, _)) => return Err((status, err))
        }
    }

    Err((StatusCode::SERVICE_UNAVAILABLE, ErrorTypes::ServerDown))
}

// pub fn str_or_default_if_exists_in_vec(s: &str, v: Vec<String>, default: &str) -> String {
//     let st = s.to_string().to_lowercase();
    
//     match v.contains(&st){
//         true => st,
//         false => default.to_string(),
//     }
// }
