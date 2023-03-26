use reqwest::StatusCode;

use crate::{actions::EClientTesting, handlers::{libs::index_exists, errors::ErrorTypes}};

/// Checks if the elastic server is up
pub async fn is_server_up(client: &EClientTesting) -> bool {
    match client.check_index("1").await {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

/// Checks if 1. Server is up, 2. App and Index exists
pub async fn check_server_up_exists_app_index(app_id: &str, index: &str, client: &EClientTesting) -> Result<(), (StatusCode, ErrorTypes)>{

    // let (server_up, app_index_exists) = futures::join!(is_server_up(client), index_exists(app_id, index, &client));



    if is_server_up(client).await {
        match index_exists(app_id, index, &client).await {
            Ok(_) => return Ok(()),
            Err((status, err, _)) => return Err((status, err))
        }
    }

    return Err((StatusCode::SERVICE_UNAVAILABLE, ErrorTypes::ServerDown))

    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    
    // match index_exists(&data.app_id, &data.index, &client).await{
    //     Ok(_) => (),
    //     Err((x, y, _)) => return HttpResponse::build(x).json(json!({"error": y.to_string()}))
    // };
}

// pub fn str_or_default_if_exists_in_vec(s: &str, v: Vec<String>, default: &str) -> String {
//     let st = s.to_string().to_lowercase();
    
//     match v.contains(&st){
//         true => st,
//         false => default.to_string(),
//     }
// }
