use actix_web::{HttpResponse, web::{self, Data}};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{actions::client::EClient, handlers::errors::ErrorTypes, AppConfig};

use super::libs::{{create_or_exists_index, is_server_up, insert_new_app_name, search_body_builder, get_document}};
use super::structs::applications_struct::*;

/// Consists of
/// An index with a list of application ids (formerly index)

/// Each application ids are index, with each containing an index name, shard number, and a prefix of application id
/// Each index (not application ids) are sharded into multiple pieces, so there are for example 10 shards
/// These contain the actual data, which were split to speed up searching

/// layout:
/*
    application_list: [
        "<app_id_1>": {
            "name": [String],
            "indexes": [Vec<String>]
        },
        ...
    ],
    app_id_1.index_name_1.0: [
        <data>,
        <data>,
        ...
    ],
    app_id_1.index_name_1.1: [
        <data>,
        <data>,
        ...
    ],
    app_id_1.index_name_2.2: [
        <data>,
        <data>,
        ...
    ],
    ...
*/

// Since there must always be an application list, this will always create one if it doesnt exist

pub async fn initialize_new_app_id(data: web::Json<RequiredAppName>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse{
    println!("Route: Initialize new app id");
    // If not exist, create a new index called Application_List containing the list of application ids
    // Generate unique id for each application ids
    // Add them to Application_List
    // Create a new index with that particular ID
    // Return status 201 if created, 409 if already exists

    if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }

    // Checks if the index "application_list" already exist, if not, create
    let _ = create_or_exists_index(None, &app_config.application_list_name, None, None, None, &client, &app_config).await;

    let app_id_status = insert_new_app_name(&data.app_name, &client, &app_config).await;

    match app_id_status {
        StatusCode::CREATED => {
            HttpResponse::Created().finish()
        },
        StatusCode::CONFLICT => {
            HttpResponse::Conflict().finish()
        },
        _ => {
            HttpResponse::build(app_id_status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }
}

pub async fn get_application_list(data: web::Path<OptionalAppName>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse{
    println!("Route: Get App List");

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}
    // If not exist, return an array of nothing
    // If there is, return a json list of the application id, its names, and the number of index it has
    // Probably use the search function in documents

    let body = search_body_builder(&data.app_name.clone(), &None, &Some("_id,name,indexes".to_string()));
    let json_resp = client.search_index(&app_config.application_list_name, &body, &None, &None).await.unwrap().json::<Value>().await.unwrap();
    if json_resp["hits"]["hits"].is_null() {
        return HttpResponse::Ok().json(json!([]))
    }
    HttpResponse::Ok().json(json!(
        json_resp["hits"]["hits"]
    ))
}


pub async fn get_application(data: web::Path<RequiredAppID>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse{
    println!("Route: Get App");
    // Search app id with /:app_id
    // If not exist, return 404
    // If there is, return application id, name, and indexes
    // This uses documents get

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    match get_document(&app_config.application_list_name, &data.app_id, &Some("_id,name,indexes".to_string()), &client).await {
        Ok((code, value)) => HttpResponse::build(code).json(value),
        Err((code, error)) => HttpResponse::build(code).json(json!({"error": error.to_string()})) 
    }
}

// 404 or 200
// Convert to proper input 
// Updates the name of an application
pub async fn update_application(data: web::Json<UpdateApp>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse{
    println!("Route: Update App");
    // Updates the name of the application
    // This uses document update

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    let body = json!({
        "doc": {
            "name": &data.app_name
        }
    });

    let resp = client.update_document(&app_config.application_list_name, &data.app_id, &body).await.unwrap();

    HttpResponse::build(resp.status_code()).finish()
}

pub async fn delete_application(data: web::Path<RequiredAppID>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse{
    println!("Route: Delete App");
    // Deletes application inside application_list
    // If not exist, return 404 
    // If there is, 
    // 1. Delete application inside application list -> Document Delete
    // 2. Delete all the index shards with application id before it -> Index Delete

    if !is_server_up(&client).await { return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({"error": ErrorTypes::ServerDown.to_string()}))}

    let resp = client.delete_document(&app_config.application_list_name, &data.app_id).await.unwrap();
    let status = resp.status_code();
    if status.is_success(){
        let indexes = client.cat_get_index(Some(format!("{}.*", data.app_id.to_ascii_lowercase()))).await.unwrap().json::<Vec<Value>>().await.unwrap();
            
        for i in indexes {
            let _ = client.delete_index(i["index"].as_str().unwrap()).await;
        }
    } else {
        return HttpResponse::NotFound().json(json!({"error": ErrorTypes::ApplicationNotFound(data.app_id.to_owned()).to_string()}))
    }
       
    HttpResponse::build(status).finish()
}