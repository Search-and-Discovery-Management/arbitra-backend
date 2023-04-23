use actix_web::{web::{self, Data}, HttpResponse};
// use nanoid::nanoid;
use reqwest::StatusCode;
use serde_json::{json, Value};
use crate::{actions::EClient, handlers::{libs::{index_name_builder, search_body_builder, bulk_create}, errors::ErrorTypes}};
use super::libs::{check_server_up_exists_app_index, document_search};
use super::structs::{document_struct::{DocumentSearchQuery, RequiredDocumentID, ReturnFields}, index_struct::RequiredIndex};

/// Document interfaces with index that is stored within the application id
/// Inserting a document with a new field syncs the fields with all other shards
/// 
/// All operations requires app_id and the index name
pub async fn create_bulk_documents(app_index: web::Path<RequiredIndex>, data: web::Json<Vec<Value>>, client: Data::<EClient>) -> HttpResponse {
    println!("Route: Bulk Create Document");

    let idx = app_index.index.clone().trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    bulk_create(&app_index.app_id, &app_index.index, &data, &client).await
}

pub async fn get_document(data: web::Path<RequiredDocumentID>, query: web::Path<ReturnFields>, client: Data::<EClient>) -> HttpResponse {  
    println!("Route: Get Document");
    // App id, index name, and document id
    // This will retrieve the shard number appended on the id then retrieve document located on that shard

    let idx = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };

    let doc_split: Vec<_> = data.document_id.split('.').collect();

    let shard_number = doc_split.last();

    let name = &format!("{}.{}", index_name_builder(&data.app_id, &idx), shard_number.unwrap());

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
pub async fn post_search(app_index: web::Path<RequiredIndex>, optional_data: Option<web::Json<DocumentSearchQuery>>, client: Data::<EClient>) -> HttpResponse {
    println!("Route: POST Search");

    let data = if optional_data.is_some(){
        optional_data.as_deref().unwrap().to_owned()
    } else {
        &DocumentSearchQuery{
            search_term: None,
            search_in: None,
            return_fields: None,
            from: None,
            count: None,
            wildcards: None
        }
    };

    let total_time_taken = std::time::Instant::now();


    let idx = app_index.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let fields_to_search = data.search_in.to_owned().map(|val| val.split(',').map(|x| x.trim().to_string()).collect());

    let term = if data.search_term.is_some() && data.wildcards.unwrap_or(false){
        let mut z = data.search_term.clone().unwrap().trim().to_string().replace(' ', "* ");
        z.push('*');
        Some(z)
    } else {
        data.search_term.clone()
    };
    let body = search_body_builder(&term, &fields_to_search, &data.return_fields);

    match document_search(&app_index.app_id, &idx, &body, &data.from, &data.count, true, &client).await {
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
pub async fn search(app_index: web::Path<RequiredIndex>, data: web::Query<DocumentSearchQuery>, client: Data::<EClient>) -> HttpResponse {
    println!("Route: Search");

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

    match document_search(&app_index.app_id, &idx, &body, &data.from, &data.count, true, &client).await {
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

pub async fn update_document(app_index_doc: web::Path<RequiredDocumentID>, data: web::Json<Value>, client: Data::<EClient>) -> HttpResponse {  
    println!("Route: Update Document");
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Updates the documents in shard

    let idx = app_index_doc.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index_doc.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let doc_split: Vec<_> = app_index_doc.document_id.split('.').collect();

    let shard_number = doc_split.last();

    let name = &format!("{}.{}", index_name_builder(&app_index_doc.app_id, &idx), shard_number.unwrap());
    
    let doc = json!({
        "doc": &data
    });

    let resp = client.update_document(name, &app_index_doc.document_id, &doc).await.unwrap();
    
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

pub async fn delete_document(data: web::Path<RequiredDocumentID>, client: Data::<EClient>) -> HttpResponse {  
    println!("Route: Delete Document");
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Deletes the document in shard

    let idx = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };

    let doc_split: Vec<_> = data.document_id.split('.').collect();

    let shard_number = doc_split.last();

    let name = &format!("{}.{}", index_name_builder(&data.app_id, &idx), shard_number.unwrap());

    let resp = client.delete_document(name, &data.document_id).await.unwrap();

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