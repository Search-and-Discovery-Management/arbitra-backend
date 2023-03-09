use actix_web::{HttpResponse, web::{self, Data}};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{actions::client::EClientTesting, handlers::errors::ErrorTypes, APPLICATION_LIST_NAME};

use super::{libs::{create_or_exists_index, is_server_up, insert_new_app_name, search_body_builder, get_document}};

// TODO: Replace every input with serde struct

/// Consists of
/// An index with a list of application ids (formerly index)

/// Each application ids are index, with each containing an index name, shard number, and a prefix of application id
/// Each index (not application ids) are sharded into multiple pieces, so there are for example 10 shards
/// These contain the actual data, which were split to speed up searching

/// layout:
/*  {
        "application_id_list": {
            "app_ids": ["application_id_1", "application_id_2"]
        },
        "application_id_1": {
            "[nama_index_1]": {
                "shards": ["application_id_1_nama_index_1_shard_1","application_id_1_nama_index_1_shard_2","application_id_1_nama_index_1_shard_3"]
            }
        },
        "application_id_2": ...,
        "application_id_1_nama_index_1_shard_1": {
            "data_1": {},
            "data_2": {},
            "data_3": {},
            ...
        },
        "application_id_1_nama_index_1_shard_2": ...
    }
*/
/// OR 
/// Need a way to get the list of application ids from serde json
/*  {
        "application_id_list": {
            "application_id_1": {
                "name": "app_1_name",
                "index_list": ["1","2",...]
            },
            "application_id_2": {
                "name": "app_2_name",
                "index_list": ["1","2",...]
            },
            ...
        },
        "application_id_1_nama_index_1_shard_1": {
            "data_1": {},
            "data_2": {},
            "data_3": {},
        },
        "application_id_1_nama_index_1_shard_2": ...
    }
*/

// Since there must always be an application list, this will always create one if it doesnt exist
pub async fn initialize_new_app_id(data: web::Json<Value>, client: Data::<EClientTesting>) -> HttpResponse{
    // If not exist, create a new index called Application_List containing the list of application ids
    // Generate unique id for each application ids
    // Add them to Application_List
    // Create a new index with that particular ID
    // Return status 201 if created, 409 if already exists

    let dat = data.into_inner();

    let app_name = dat.get("app_name").unwrap().as_str().unwrap();

    if !is_server_up(&client).await {
        return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()}))
    }

    // Checks if the index "application_list" already exist, if not, create
    let _ = create_or_exists_index(APPLICATION_LIST_NAME, None, None, &client).await;

    let app_id_status = insert_new_app_name(app_name, APPLICATION_LIST_NAME, &client).await;

    match app_id_status {
        StatusCode::CREATED => {
            return HttpResponse::Created().finish()
        },
        StatusCode::CONFLICT => {
            return HttpResponse::Conflict().finish()
        },
        _ => {
            return HttpResponse::build(app_id_status).json(json!({"error": ErrorTypes::Unknown.to_string()}));
        }
    }
}

pub async fn get_application_list(data: web::Json<Value>, client: Data::<EClientTesting>) -> HttpResponse{
    // If not exist, return an array of nothing
    // If there is, return a json list of the application id, its names, and the number of index it has
    // Probably use the search function in documents

    // Potential search fields replacement: Doc value fields, but still loads from disk
    // _source is slow as it loads everything from disk, which is then filtered
    // fields arg retrieves only selected fields, but always as an array
    // let body = json!({
    //     "_source": false,
    //     "query": {
    //         "match_all": {} 
    //     },
    //     "fields": ["name", "indexes"]
    // });

    // TODO: Use serde json deserialize 
    let dat = data.into_inner();
    let app_name = dat.get("app_name").unwrap().as_str().unwrap().to_string();
    let body = search_body_builder(Some(app_name), None, Some("_id,name,indexes".to_string()), false, Some("AUTO".to_string()));
    let json_resp = client.search_index(APPLICATION_LIST_NAME, body, None, None).await.unwrap().json::<Value>().await.unwrap();
    HttpResponse::Ok().json(json!({
        "took": json_resp["took"],
        "data": json_resp["hits"]["hits"],
        "total_data": json_resp["hits"]["total"]["value"],
    }))
}

pub async fn get_application(data: web::Path<String>, client: Data::<EClientTesting>) -> HttpResponse{
    // Search app id with /:app_id
    // If not exist, return 404
    // If there is, return application id, name, and indexes
    // This uses documents get

    let dat = data.into_inner();
    let resp = get_document(APPLICATION_LIST_NAME, &dat, Some("_id,name,indexes".to_string()), &client).await;

    match resp {
        Ok((code, value)) => return HttpResponse::build(code).json(value),
        Err((code, error)) => return HttpResponse::build(code).json(json!({"error": error.to_string()})) 
    };
}

// 404 or 200
// Convert to proper input 
// This simply updates the name, without adding or deleting
pub async fn update_application(data: web::Json<Value>, client: Data::<EClientTesting>) -> HttpResponse{
    // Updates the name of the application
    // This uses document update

    let dat = data.into_inner();

    let app_id = dat.get("app_id").unwrap().as_str().unwrap();
    let name = dat.get("name").unwrap().as_str().unwrap();

    let body = json!({
        "name": name
    });

    let resp = client.update_document(APPLICATION_LIST_NAME, app_id, body).await.unwrap();

    HttpResponse::build(resp.status_code()).finish()
}

// TODO: Actually delete the indexes with app_id of app being deleted
pub async fn delete_application(data: web::Path<String>, client: Data::<EClientTesting>) -> HttpResponse{
    // Deletes application inside application_list
    // If not exist, return 404 
    // If there is, 
    // 1. Delete all the index shards with application id before it -> Index Delete
    // 2. Delete application inside application list -> Document Delete

    let app_id = data.into_inner();

    let resp = client.delete_document(APPLICATION_LIST_NAME, &app_id).await.unwrap();

    HttpResponse::build(resp.status_code()).finish()
}