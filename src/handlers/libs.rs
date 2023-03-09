// Contains the functions shared among handler functions

use actix_web::web::Redirect;
use elasticsearch::GetParts;
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{actions::client::EClientTesting};

use super::errors::ErrorTypes;

/// Checks if the elastic server is up
pub async fn is_server_up(client: &EClientTesting) -> bool {
    match client.check_index("1").await {
        Ok(_) => return true,
        Err(_) => return false,
    }
    
}

// TODO: Redo for proper error handling
/// Checks if the app name exists
pub async fn exists_app_name(app_name: &str, list_name: &str, client: &EClientTesting) -> bool{
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
    
    // TODO: Possibly flawed search since it may find ones with similar name with exact keywords
    // let body = json!({
    //     "_source": false,
    //         "query": {
    //             "multi_match": {
    //                 "query": app_name,
    //                 "fields": "name",
    //                 "fuzziness": 0     
    //             }
    //         },
    //     });
    let body = search_body_builder(Some(app_name.to_string()), Some("name".to_string()), None, false, Some("0".to_string()));

    let resp = client.search_index(list_name, body, None, Some(1)).await.unwrap();
    let resp_json = resp.json::<Value>().await.unwrap();
    println!("{:#?}", resp_json);
    
    let num = resp_json["hits"]["total"]["value"].as_i64().unwrap();

    if num > 0 {
        return true
    } else {
        return false
    }
}

/// Inserts a new app name to the application list
pub async fn insert_new_app_name(app_name: &str, list_name: &str, client: &EClientTesting) -> StatusCode {
    let exists = exists_app_name(app_name, list_name, client).await;

    // If exists, return conflict
    if exists {
        return StatusCode::CONFLICT;
    }

    let body = json!({
        "name": app_name,
        "indexes": []
    });

    // Inserts name into app_id
    client.insert_document(list_name, body).await.unwrap().status_code()
}

/// Inserts a new index into an application
pub async fn create_or_exists_index(index: &str, shards: Option<i64>, replicas: Option<i64>, client: &EClientTesting) -> StatusCode {

    // Check if index exists
    let exists = client.check_index(index).await.unwrap();

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
        let resp = client.create_index(body, index).await.unwrap();

        if resp.status_code() == StatusCode::OK {
            return StatusCode::CREATED;
        }
    }

    exists.status_code()
}

// TODO: Convert into a lib function that uses actions
pub async fn get_document(index: &str, document_id: &str, retrieve_fields: Option<String>, client: &EClientTesting) -> Result<(StatusCode, Value), (StatusCode, ErrorTypes)>{
    let resp = client.get_document(index.to_string(), document_id.to_string(), retrieve_fields).await.unwrap();

    let status_code = resp.status_code();
    
    if !status_code.is_success() {
        let error = match status_code{
            StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(document_id.to_string()),
            _ => ErrorTypes::Unknown
        };
        return Err((status_code, error));
    }

    let json_resp = resp.json::<Value>().await.unwrap();

    Ok((status_code, json_resp))
}

pub fn search_body_builder(search_term: Option<String>, search_in: Option<String>, retrieve_field: Option<String>, include_source: bool, fuzziness: Option<String>) -> Value{
    let fields_to_search: Option<Vec<String>> = search_in.map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());

    let fields_to_return = match retrieve_field {
        Some(val) => val.split(',').into_iter().map(|x| x.trim().to_string()).collect(),
        None => vec!["*".to_string()],
    };

    // Returns everything
    let mut body = json!({
        "_source": include_source,
        "query": {
            "match_all": {} 
        },
        "fields": fields_to_return
    });

    // Some(temp_variable) = existing_variable {function(temp_variable from existing_variable)}
    if let Some(search) = search_term {
        if let Some(search_field) = fields_to_search {
            body = json!({
                "_source": include_source,
                "query": {
                    "multi_match": {
                        "query": search,
                        "fields": search_field,
                        "fuzziness": fuzziness.unwrap_or("AUTO".to_string())     
                    }
                },
                "fields": fields_to_return
            })
        } else {
            body = json!({
                "_source": include_source,
                "query": {
                    "multi_match": {
                        "query": search,
                        "fields": "*",
                        "fuzziness": fuzziness.unwrap_or("AUTO".to_string())     
                    }
                },
                "fields": fields_to_return
            });
        }
    };

    return body;
}
