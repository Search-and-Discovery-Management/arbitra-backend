use actix_web::{web::{self, Data}, HttpResponse};
use reqwest::StatusCode;
use serde_json::{json, Value};
use crate::{actions::EClientTesting, handlers::{libs::{index_name_builder, search_body_builder}, errors::ErrorTypes}};
use super::libs::{check_server_up_exists_app_index, document_search};
use super::structs::{document_struct::{DocumentSearchQuery, RequiredDocumentID, ReturnFields, BulkFailures}, index_struct::RequiredIndex};

/// Document interfaces with index that is stored within the application id
/// Inserting a document with a new field syncs the fields with all other shards
/// 
/// All operations requires app_id and the index name
/// 

pub async fn create_bulk_documents(app_index: web::Path<RequiredIndex>, data: web::Json<Vec<Value>>, client: Data::<EClientTesting>) -> HttpResponse {

    let idx = app_index.index.clone().trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let name = index_name_builder(&app_index.app_id, &idx);

    let resp = client.bulk_index_documents(&name, &data).await.unwrap();

    let status = resp.status_code();
    let json: Value = resp.json::<Value>().await.unwrap();

    let mut failures: Vec<BulkFailures> = vec![];
    if json["errors"].as_bool().unwrap() {
        for (loc, val) in json["items"].as_array().unwrap().iter().enumerate(){
            if !val["index"]["error"].is_null(){
                failures.push(
                    BulkFailures {
                        document_number: loc,
                        error: val["index"]["error"]["reason"].as_str().unwrap().to_string(),
                        status: val["index"]["status"].as_i64().unwrap()
                    }
                );
            }
        }
    }
    
    HttpResponse::build(status).json(serde_json::json!({
        "error_count": failures.len(),
        "has_errors": json["errors"].as_bool().unwrap(),
        "errors": failures
    }))
}

pub async fn get_document(data: web::Path<RequiredDocumentID>, query: web::Path<ReturnFields>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // App id, index name, and document id
    // This will retrieve the shard number appended on the id

    let idx = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };

    let name = &index_name_builder(&data.app_id, &idx);

    let resp = client.get_document(name, &data.document_id, &query.return_fields).await.unwrap();

    let status_code = resp.status_code();
    
    if !status_code.is_success() {
        let error = match status_code{
            StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(data.document_id.to_string()).to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };
        return HttpResponse::build(status_code).json(json!({"error": error}));
    }

    let convert_time = std::time::Instant::now();
    let json_resp = resp.json::<Value>().await.unwrap();
    println!("{:#?}", convert_time.elapsed().as_millis());

    HttpResponse::build(status_code).json(json_resp)
}

/// A Post method for search, also returns a list of documents from index if no query is given
pub async fn post_search(app_index: web::Path<RequiredIndex>, data: web::Json<DocumentSearchQuery>, client: Data::<EClientTesting>) -> HttpResponse {

    let total_time_taken = std::time::Instant::now();


    let idx = app_index.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    // TODO: Default search_in into a type of searchableAttributes which defaults its search to all fields with searchableAttributes when nothing is supplied 
    let fields_to_search = data.search_in.to_owned().map(|val| val.split(',').map(|x| x.trim().to_string()).collect());

    let term = if data.search_term.is_some() && data.wildcards.unwrap_or(false){
        let mut z = data.search_term.clone().unwrap().trim().to_string().replace(' ', "* ");
        z.push('*');
        Some(z)
    } else {
        data.search_term.clone()
    };
    let body = search_body_builder(&term, &fields_to_search, &data.return_fields);

    match document_search(&app_index.app_id, &idx, &body, &data.from, &data.count, &client).await {
        Ok((status, json_resp)) => {
            HttpResponse::build(status).json(json!({
                "search_took": &json_resp["took"],
                "total_took": &total_time_taken.elapsed().as_millis(),
                "data": &json_resp["hits"]["hits"],
                "total_data": &json_resp["hits"]["total"]["value"],
                "match_type": &json_resp["hits"]["total"]["relation"],
                "from": &data.from.unwrap_or(0),
                "count": &data.count.unwrap_or(20)
            }))
        },
        Err(err) => err,
    }
}

/// A Get method for search, also returns a list of documents from index if no query is given
pub async fn search(app_index: web::Path<RequiredIndex>, data: web::Query<DocumentSearchQuery>, client: Data::<EClientTesting>) -> HttpResponse {

    let total_time_taken = std::time::Instant::now();

    let idx = app_index.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let fields_to_search = data.search_in.to_owned().map(|val| val.split(',').map(|x| x.trim().to_string()).collect());

    let term = if data.search_term.is_some() && data.wildcards.unwrap_or(false){
        let mut z = data.search_term.as_deref().unwrap().trim().to_string().replace(' ', "* ");
        z.push('*');
        Some(z)
    } else {
        data.search_term.clone()
    };

    let body = search_body_builder(&term, &fields_to_search, &data.return_fields);

    match document_search(&app_index.app_id, &idx, &body, &data.from, &data.count, &client).await {
        Ok((status, json_resp)) => {
            HttpResponse::build(status).json(json!({
                "search_took": &json_resp["took"],
                "total_took": &total_time_taken.elapsed().as_millis(),
                "data": &json_resp["hits"]["hits"],
                "total_data": &json_resp["hits"]["total"]["value"],
                "match_type": &json_resp["hits"]["total"]["relation"],
                "from": &data.from.unwrap_or(0),
                "count": &data.count.unwrap_or(20)
            }))
        },
        Err(x) => x,
    }
}

pub async fn update_document(app_index_doc: web::Path<RequiredDocumentID>, data: web::Json<Value>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Updates the documents in shard

    let idx = app_index_doc.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index_doc.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let name = index_name_builder(&app_index_doc.app_id, &idx);
    
    let doc = json!({
        "doc": &data
    });

    let resp = client.update_document(&name, &app_index_doc.document_id, &doc).await.unwrap();
    
    let status = resp.status_code();
    
    if !status.is_success() {
        let error = match status{
            StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(app_index_doc.document_id.to_string()).to_string(),
            StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };
        return HttpResponse::build(status).json(json!({"error": error}));
    }

    HttpResponse::build(status).finish()
}

pub async fn delete_document(data: web::Path<RequiredDocumentID>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Deletes the document in shard

    let idx = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };

    let name = index_name_builder(&data.app_id, &idx);

    let resp = client.delete_document(&name, &data.document_id).await.unwrap();

    let status_code = resp.status_code();

    if !status_code.is_success() {
        let error = match status_code{
            StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(data.document_id.to_string()).to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };
        return HttpResponse::build(status_code).json(json!({"error": error}));
    }

    HttpResponse::build(status_code).finish()
}