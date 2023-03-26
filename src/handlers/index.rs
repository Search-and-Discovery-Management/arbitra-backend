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
pub async fn _create_index(data: web::Json<IndexCreate>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }

    // match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
    //     Ok(_) => (),
    //     Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    // };

    if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})); };

    // let app_exists = get_document(APPLICATION_LIST_NAME, &data.app_id, Some("".to_string()), &client).await;

    // if app_exists.is_err() {
    //     return HttpResponse::NotFound().json(json!({"error": ErrorTypes::ApplicationNotFound(data.app_id.clone()).to_string()}));
    // }

    // let idx_list = get_app_indexes_list(&data.app_id, &client).await.unwrap();

    // match idx_list.iter().position(|x| x.eq(&data.index)) {
    //     Some(_) => return HttpResponse::Conflict().finish(),
    //     None => ()
    // }
    

    // match index_exists(&data.app_id, &data.index, &client){
    //     Ok(_) => HttpResponse,
    //     Err(_) => (),
    // }

    // Adds index to an application id, then creates a number of new index 

    // Gets the app's current indexes, append the new index, then update it, then create 10 shards

    // if get_document returns 404, then app id doesnt exist, if there is but "indexes" field doesnt exist, then put a new one

    // let resp = client.get_document(APPLICATION_LIST_NAME, &data.app_id, Some("indexes".to_string())).await;

    // let resp_json = match resp {
    //     Ok(x) => {
    //         x.json::<Value>().await.unwrap()
    //     },
    //     Err(x) => {
    //         return HttpResponse::build(x.status_code().unwrap()).json(json!({"error": x.to_string()}))
    //     }
    // };

    // let mut list: Vec<String> = match resp_json.get("indexes") {
    //     Some(x) => serde_json::from_value(x.clone()).unwrap(),
    //     None => Vec::new()
    // };

    // let exists = list.iter().position(|x| x.eq(&data.index));
    return match index_exists(&data.app_id, &data.index, &client).await {
        // Ok((_, x))
        // Err(_, x) => x,
        // None => return HttpResponse::build(status).json(json!({"error": error.to_string()})),
        // If exists, return, else, create index
        Ok(_) => HttpResponse::Conflict().json(json!({"error": ErrorTypes::IndexExists(data.index.to_string()).to_string()})),
        Err((status, error, mut list)) => match error {
            ErrorTypes::ApplicationNotFound(_) => HttpResponse::build(status).json(json!({"error": error.to_string()})),
            ErrorTypes::IndexNotFound(_) => {
                list.push(data.index.to_string());
                let body = json!({
                    "doc": {
                        "indexes": list
                    }
                });
                // TODO: Adding new index creates 10 shards -> lib function? -- DELAYED
                let _ = client.update_document(APPLICATION_LIST_NAME, &data.app_id, &body).await;

                let _ = create_or_exists_index(Some(data.app_id.to_string()), &data.index, data.shards, data.replicas, &client).await.to_string();

                HttpResponse::Created().finish()
            },
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    };

    // match index_exists(&data.app_id, &data.index, &client).await {
    //     Some(_) => {
    //         //Already Exists
    //         return HttpResponse::Conflict().json(json!({"error": ErrorTypes::IndexExists(data.index.to_string()).to_string()}));
    //     },
    //     None => {
    //         // Insert new index into the app's indexes
    //         list.push(data.index.to_string());
    //         let body = json!({
    //             "doc": {
    //                 "indexes": list
    //             }
    //         });
    //         // TODO: Adding new index creates 10 shards -> lib function? -- DELAYED
    //         let _ = client.update_document(APPLICATION_LIST_NAME, &data.app_id, body).await;

    //         let _ = create_or_exists_index(Some(data.app_id.to_string()), &data.index, data.shards, data.replicas, &client);

    //         return HttpResponse::Created().finish();
    //     },
    // }
}

pub async fn _get_index(app: web::Path<RequiredAppID>, idx_name: web::Query<OptionalIndex>, client: Data::<EClientTesting>) -> HttpResponse {  
    // Retrieves either one or all index from an application id, returns index or 404 if not found
    // Retrieves index from an application id, returns index or 404 if not found
    // Returns stats of the index
    // TODO: Return an aggregated result of all the shards (num of docs, deleted docs, etc) -- DELAYED
    // --TODO: Actually check if application id exists--

    match &idx_name.index {
        Some(x) => {
            match check_server_up_exists_app_index(&app.app_id, x, &client).await{
                Ok(_) => (),
                Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
            };
        },
        None => if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    }
    
    let app_id = &app.app_id;
    let index = &idx_name.index.to_owned().unwrap_or("*".to_string());
        
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
    return HttpResponse::build(status).json(json_resp);
    // match index {
    //     Some(x) => {

            
    //     },
    //     None => {
    //         let name = format!("{app_id}.*");
    //         let idx = client.get_index(Some(name)).await.unwrap();

    //         return match idx.status_code() {
    //             StatusCode::OK => HttpResponse::build(idx.status_code()).json(idx.json::<Value>().await.unwrap()),
    //             StatusCode::NOT_FOUND => HttpResponse::NotFound().json(idx.json::<Value>().await.unwrap()),
    //             _ => HttpResponse::build(idx.status_code()).json(json!({"error": ErrorTypes::Unknown.to_string()}))
    //         }
    //     }
    // };

    // let resp = client.get_document(APPLICATION_LIST_NAME, app_id, Some("indexes".to_string())).await;

    // let resp_json = match resp {
    //     Ok(x) => {
    //         x.json::<Value>().await.unwrap()
    //     },
    //     Err(x) => {
    //         return HttpResponse::build(x.status_code().unwrap()).json(json!({"error": x.to_string()}))
    //     }
    // };

    // let list: Vec<String> = match resp_json.get("indexes") {
    //     Some(x) => serde_json::from_value(x.clone()).unwrap(),
    //     None => Vec::new()
    // };

    // return HttpResponse::Ok().json(json!({"indexes": list}));
}

// TODO: Actually test function
pub async fn _get_mappings(data: web::Path<RequiredIndex>, client: Data::<EClientTesting>) -> HttpResponse {
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    // Returns the mappings of the index
    // TODO: Actually check if application id exists
        
    let name = index_name_builder(&data.app_id, &data.index);
    let idx = client.get_index_mappings(&name).await.unwrap();

    let status = idx.status_code();

    if !status.is_success(){
        return match status {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().finish(),
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }

    println!("{:#?}", idx);

    // let json_resp = 
    //         resp.json::<Value>()
    //         .await
    //         .unwrap();


    //     let mappings = json_resp
    //         .get(&index)
    //         .unwrap()
    //         .get("mappings")
    //         .unwrap();

    let json_resp = idx.json::<Value>().await.unwrap();
    return HttpResponse::build(status).json(json_resp[&name].clone());
}

// TODO: Actually test function
pub async fn _update_mappings(data: web::Json<IndexMappingUpdate>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    // Updates the mappings of the index
    // TODO: Actually check if application id exists
        
    let name = index_name_builder(&data.app_id, &data.index);
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
pub async fn _delete_index(data: web::Path<RequiredIndex>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }

    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    // Deletes index along with its shard, then removes itself from the application id's index list

    let app_id = &data.app_id;
    let index = &data.index;

    let name = index_name_builder(app_id, index);
    let idx = client.delete_index(&name).await.unwrap();

    let status = idx.status_code();

    if !status.is_success(){
        return match status {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().finish(),
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()}))
        }
    }

    return match index_exists(app_id, index, &client).await {
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