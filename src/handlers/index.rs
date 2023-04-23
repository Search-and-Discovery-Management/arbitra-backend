use actix_web::{web::{self, Data}, HttpResponse};
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{actions::EClient, APPLICATION_LIST_NAME};

use super::structs::{index_struct::*, applications_struct::RequiredAppID};
use super::libs::{index::index_exists, create_or_exists_index, index_name_builder, is_server_up, check_server_up_exists_app_index, get_app_indexes_list};
use super::errors::*;

/// Index interfaces with application_id
/// Creating a new index accesses application_list which finds application_id of that specific index, then adds a new index to the id's list
/// TODO: Do not allow index name with space, dots, etc and allow only alphabets, numbers, and underscores
/// 
/// Creates x amount of partition indexes such that (default: 10)
/// app_id.index_name.partition_number
pub async fn create_index(app: web::Path<RequiredAppID>, data: web::Json<IndexCreate>, client: Data::<EClient>) -> HttpResponse {  
    println!("Route: Create Index");

    if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})); };

    // Adds index to an application id, then creates a number of new index 
    // if get_document returns 404, then app id doesnt exist, if there is but "indexes" field doesnt exist, then put a new one

    let idx = data.index.trim().to_ascii_lowercase().replace(' ', "_");

    match index_exists(&app.app_id, &idx, &client).await {
        // If exists, return, else, create index
        Ok(_) => HttpResponse::Conflict().json(json!({"error": ErrorTypes::IndexExists(idx).to_string()})),
        Err((status, error, mut list)) => match error {
            ErrorTypes::ApplicationNotFound(_) => HttpResponse::build(status).json(json!({"error": error.to_string()})),
            ErrorTypes::IndexNotFound(_) => {
                list.push(idx.clone());
                list.sort();
                list.dedup();
                let body = json!({
                    "doc": {
                        "indexes": list
                    }
                });
                let _ = client.update_document(APPLICATION_LIST_NAME, &app.app_id, &body).await;

                let x = match data.partitions {
                    Some(z) => Some(z),
                    None => Some(10)
                };

                let _ = create_or_exists_index(Some(app.app_id.to_string()), &idx, data.shards, data.replicas, x, &client).await.to_string();

                HttpResponse::Created().finish()
            },
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }
}

// TODO: Refactor This
pub async fn get_index(app: web::Path<RequiredAppID>, idx_name: web::Query<OptionalIndex>, client: Data<EClient>) -> HttpResponse{
    println!("Route: Get Index");

    let resp = client.get_document(APPLICATION_LIST_NAME, &app.app_id, &Some("indexes".to_string())).await.unwrap();

    let status = resp.status_code();

    if !status.is_success(){
        return HttpResponse::NotFound().json(json!({"error": ErrorTypes::ApplicationNotFound(app.app_id.to_owned()).to_string()}))
    }
    
    match get_app_indexes_list(&app.app_id, &client).await {
        Ok(x) => {
            let mut indexes: Vec<IndexResponse> = vec![];
            if idx_name.index.is_some(){
                if x.contains(&idx_name.index.clone().unwrap()){
                    let name = format!("{}.*", index_name_builder(&app.app_id, &idx_name.index.clone().unwrap()));
                    let idx = client.get_index_stats(&[&name], Some(&["_all"]), Some(&["_all.primaries.docs", "_all.primaries.store"])).await.unwrap().json::<Value>().await.unwrap();
                    if idx.get("_all").is_some(){
                        let count = idx["_all"]["primaries"]["docs"].get("count").unwrap();
                        let deleted = idx["_all"]["primaries"]["docs"].get("deleted").unwrap();
                        let size = idx["_all"]["primaries"]["store"].get("size_in_bytes").unwrap();
    
                        indexes.push(
                            IndexResponse{
                                index: idx_name.index.clone().unwrap(),
                                docs_count: count.as_u64().unwrap(),
                                docs_deleted: deleted.as_u64().unwrap(),
                                primary_size: size.as_u64().unwrap()
                            }
                        )
                    }
                } else {
                    return HttpResponse::NotFound().json(json!({"error": ErrorTypes::IndexNotFound(idx_name.index.clone().unwrap()).to_string()}))
                }
            } else {
                for i in x {
                    let name = format!("{}.*", index_name_builder(&app.app_id, &i));
                    let idx = client.get_index_stats(&[&name], Some(&["_all"]), Some(&["_all.primaries.docs", "_all.primaries.store"])).await.unwrap().json::<Value>().await.unwrap();

                    if idx.get("_all").is_some(){
                        let count = idx["_all"]["primaries"]["docs"].get("count").unwrap();
                        let deleted = idx["_all"]["primaries"]["docs"].get("deleted").unwrap();
                        let size = idx["_all"]["primaries"]["store"].get("size_in_bytes").unwrap();

                        indexes.push(
                            IndexResponse{
                                index: i,
                                docs_count: count.as_u64().unwrap(),
                                docs_deleted: deleted.as_u64().unwrap(),
                                primary_size: size.as_u64().unwrap()
                            }
                        )
                    }
                }
            }
            HttpResponse::Ok().json(indexes)
        },
        Err((status, err)) => HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }
}


pub async fn get_app_list_of_indexes(app: web::Path<RequiredAppID>, client: Data::<EClient>) -> HttpResponse {  
    println!("Route: Get App Indexes List");
    // Gets the list of indexes in an application

    if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) };

    match get_app_indexes_list(&app.app_id, &client).await {
        Ok(list) => HttpResponse::Ok().json(json!(list)),
        Err((status, err)) => HttpResponse::build(status).json(json!({"error": err.to_string()}))
    }
}

// Gets shard 0 mappings
pub async fn get_mappings(data: web::Path<RequiredIndex>, client: Data::<EClient>) -> HttpResponse {
    println!("Route: Get Mappings, app id:{}, index mapping being updated: {}", data.app_id, data.index);

    let index = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    // Returns the mappings of the index
        
    let name = format!("{}.0", index_name_builder(&data.app_id, &index));
    let idx = client.get_index_mappings(&name).await.unwrap();

    let status = idx.status_code();

    if !status.is_success(){
        return match status {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().finish(),
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }

    println!("{:#?}", idx);

    let json_resp = idx.json::<Value>().await.unwrap();
    HttpResponse::build(status).json(json_resp[&name].clone())
}

// Updates all shard mappings
pub async fn update_mappings(data: web::Json<IndexMappingUpdate>, client: Data::<EClient>) -> HttpResponse { 
    println!("Route: Update Mappings, app id:{}, index mapping being updated: {}", data.app_id, data.index);
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    let index = data.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&data.app_id, &index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    // Updates the mappings of the index
        
    let name = format!("{}.*", index_name_builder(&data.app_id, &index));
    let idx = client.update_index_mappings(&name, &data.mappings).await.unwrap();

    let status = idx.status_code();
    println!("{:#?}", idx.json::<Value>().await.unwrap());
    if !status.is_success(){
        return match status {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().finish(),
            StatusCode::BAD_REQUEST => HttpResponse::BadRequest().json(json!({"error": ErrorTypes::BadDataRequest.to_string()})),
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }

    HttpResponse::build(status).finish()
}

// Remove from app id then delete all shard
pub async fn delete_index(data: web::Path<RequiredIndex>, client: Data::<EClient>) -> HttpResponse {  
    println!("Route: Delete Index, app id:{}, index being deleted: {}", data.app_id, data.index);
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    let index = data.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&data.app_id, &index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    // Removes itself from the application id's index list, then proceed to delete the shard indexes

    match index_exists(&data.app_id, &index, &client).await {
        // If app and index exists
        Ok((needle, mut list)) => {
            list.remove(needle);
            let body = json!({
                "doc": {
                    "indexes": list
                }
            });
            
            let resp = client.update_document(APPLICATION_LIST_NAME, &data.app_id, &body).await.unwrap();
            let status = resp.status_code();

            if status.is_success(){
                let indexes = client.cat_get_index(Some(format!("{}.*", index_name_builder(&data.app_id, &data.index)))).await.unwrap().json::<Vec<Value>>().await.unwrap();
                
                for i in indexes {
                    let _ = client.delete_index(i["index"].as_str().unwrap()).await;
                }
            }
            HttpResponse::build(status).finish()
        },
        // If either doesnt exist
        Err((status, error, _)) => HttpResponse::build(status).json(json!({"error": error.to_string()})),
    }
}