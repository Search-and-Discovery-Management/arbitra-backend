use actix_web::{web::{self, Data}, HttpResponse};
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{actions::EClientTesting, APPLICATION_LIST_NAME};

use super::{index_struct::*, errors::*, libs::{index::index_exists, create_or_exists_index, index_name_builder, is_server_up, check_server_up_exists_app_index}};

// Temp _ because models and routes having same name

// TODO: Output same / at least similar to API Contract

/// Index interfaces with application_id
/// Creating a new index accesses application_list which finds application_id of that specific index, then adds a new index to the id's list
/// TODO: Do not allow index name with space, dots, etc and allow only alphabets, numbers, and underscores
pub async fn create_index(app: web::Path<RequiredAppID>, data: web::Json<IndexCreate>, client: Data::<EClientTesting>) -> HttpResponse {  

    if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})); };

    // Adds index to an application id, then creates a number of new index 

    // Gets the app's current indexes, append the new index, then update it, then create 10 shards

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
                // TODO: Adding new index creates 10 shards -> lib function? -- DELAYED
                let _ = client.update_document(APPLICATION_LIST_NAME, &app.app_id, &body).await;

                let _ = create_or_exists_index(Some(app.app_id.to_string()), &idx, data.shards, data.replicas, &client).await.to_string();

                HttpResponse::Created().finish()
            },
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }
}

pub async fn get_index(app: web::Path<RequiredAppID>, idx_name: web::Query<OptionalIndex>, client: Data::<EClientTesting>) -> HttpResponse {  
    // Retrieves either one or all index from an application id, returns index or 404 if not found
    // Retrieves index from an application id, returns index or 404 if not found
    // Returns stats of the index
    // TODO: Return an aggregated result of all the shards (num of docs, deleted docs, etc) -- DELAYED
    // --TODO: Actually check if application id exists--

    match &idx_name.index {
        Some(x) => {
            match check_server_up_exists_app_index(&app.app_id, &x.trim().to_ascii_lowercase(), &client).await{
                Ok(_) => (),
                Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
            };
        },
        None => if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    }
    
    let app_id = &app.app_id;
    let index = &idx_name.index.to_owned().unwrap_or("*".to_string()).trim().to_ascii_lowercase();
        
    let name = index_name_builder(app_id, index);
    let idx = client.get_index(Some(name)).await.unwrap();

    let status = idx.status_code();

    if !status.is_success(){
        return match status {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().finish(),
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }

    let json_resp = idx.json::<Vec<Value>>().await.unwrap();
    HttpResponse::build(status).json(json_resp)
}

pub async fn get_mappings(data: web::Path<RequiredIndex>, client: Data::<EClientTesting>) -> HttpResponse {

    let index = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    // Returns the mappings of the index
        
    let name = index_name_builder(&data.app_id, &index);
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

pub async fn update_mappings(data: web::Json<IndexMappingUpdate>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    let index = data.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&data.app_id, &index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    // Updates the mappings of the index
        
    let name = index_name_builder(&data.app_id, &index);
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

// TODO: Returns 404 on success for some reason
pub async fn delete_index(data: web::Path<RequiredIndex>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    let index = data.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&data.app_id, &index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    // Deletes index along with its shard, then removes itself from the application id's index list

    let app_id = &data.app_id;

    let name = index_name_builder(app_id, &index);
    let idx = client.delete_index(&name).await.unwrap();

    let status = idx.status_code();

    if !status.is_success(){
        return match status {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().finish(),
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }

    match index_exists(app_id, &index, &client).await {
        // If app and index exists
        Ok((needle, mut list)) => {
            list.remove(needle);
            let body = json!({
                "indexes": list
            });
            let _ = client.update_document(APPLICATION_LIST_NAME, app_id, &body).await;
            HttpResponse::build(status).finish()
        },
        // If either doesnt exist
        Err((status, error, _)) => HttpResponse::build(status).json(json!({"error": error.to_string()})),
    }
}