// use elasticsearch::http::response::Response;
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{actions::EClientTesting, handlers::{errors::ErrorTypes, libs::document::search_body_builder}, APPLICATION_LIST_NAME};

use super::document::get_document;

/// Inserts a new app name to the application list
pub async fn insert_new_app_name(app_name: &str, client: &EClientTesting) -> StatusCode {
    let exists = exists_app_name(app_name, client).await;

    // If exists, return conflict
    if exists {
        return StatusCode::CONFLICT;
    }

    let body = json!({
        "name": app_name,
        "indexes": []
    });

    // Inserts name into app_id
    client.insert_document(APPLICATION_LIST_NAME, &body).await.unwrap().status_code()
}

pub async fn get_app_indexes_list(app_id: &str, client: &EClientTesting) -> Result<Vec<String>, (StatusCode, ErrorTypes)> {
    let (_, value) = match get_document(APPLICATION_LIST_NAME, app_id, &Some("indexes".to_string()), client).await{
        Ok(x) => x,
        Err((status, _)) => return match status {
            StatusCode::NOT_FOUND => Err((status, ErrorTypes::ApplicationNotFound(app_id.to_string()))),
            _ => Err((status, ErrorTypes::Unknown))
        },
    };

    let list: Vec<String> = match value.get("indexes") {
        Some(x) => serde_json::from_value(x.clone()).unwrap(),
        None => Vec::new()
    };
    Ok(list)
}

// pub async fn add_index_to_app_indexes(app_id: &str, index: &str, client: &EClientTesting) -> Result<Response,(StatusCode, ErrorTypes)> {


//     let resp = get_app_indexes_list(&app_id, &client).await;
//     match resp {
//         Ok(mut list) => {
//             list.push(index.to_string());
//             list.sort();
//             list.dedup();
//             let body = json!({
//                 "doc": {
//                     "indexes": list
//                 }
//             });
//             Ok(client.update_document(APPLICATION_LIST_NAME, app_id, &body).await.unwrap());
//         },
//         Err((status, err)) => return Err((status, err))
//     }
// }


// ? TODO: Redo for proper error handling
/// Checks if the app name exists
pub async fn exists_app_name(app_name: &str, client: &EClientTesting) -> bool{
    // This uses the document search for an exact match, if exists true, else false

    /*
    structure:
    "application_list": [
        {
            "name": "a"
            "index_list": ["1","2","3"]
        },
        ...
    ]
    */

    let app_name_exact = format!("\"{app_name}\"");
    
    // ! TODO: Possibly flawed search since it may find ones with similar name with exact keywords, although its unlikely to match when there is no space
    let body = search_body_builder(&Some(app_name_exact), &Some(vec!["name".to_string()]), &None);

    let resp = client.search_index(APPLICATION_LIST_NAME, &body, &None, &Some(1)).await.unwrap();
    let resp_json = resp.json::<Value>().await.unwrap();
    println!("{:#?}", resp_json);
    
    let num = resp_json["hits"]["total"]["value"].as_i64().unwrap();

    num > 0
}
