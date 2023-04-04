use actix_web::{web::{self, Data}, HttpResponse};
use reqwest::StatusCode;
use serde_json::{json, Value};
use crate::{actions::EClientTesting, handlers::{libs::{index_name_builder, search_body_builder, index_exists}, errors::ErrorTypes}};
use super::{document_struct::{DocumentCreate, GetDocumentSearchIndex, GetDocumentSearchQuery, DocumentSearch, DocumentUpdate, DocumentDelete, DocById, ReturnFields}, libs::{check_server_up_exists_app_index, document_search}};

/// Document interfaces with index that is stored within the application id
/// Inserting a document with a new field syncs the fields with all other shards
/// 
/// All operations requires app_id and the index name
/// 

pub async fn create_document(data: web::Json<DocumentCreate>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    let idx = data.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&data.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    // Check app exists -> check index exists
    // -> Get document

    // Creates a new document by getting application id, index name, check if document has new field, if yes, check dynamic mode
    // if true, update the entire index shards to accomodate the new field, then insert

    // TODO: Change ID to have appended shard number
    // Solutions: Either use fingerprint or use bulk api, the latter being most likely since it allows direct insert

    // Inserts document into index -> Checks if app has index
    // Checks if index exists
    // Insert

    match index_exists(&data.app_id, &idx, &client).await {
        Ok(_) => (),
        Err((status, err, _)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }
    
    let name = index_name_builder(&data.app_id, &idx);
    println!("{:#?}", name);

    let dat = &data.data;
    let dynamic_mode = &data.dynamic_mode;
    
    // If dynamic mode has value, set to whatever is inputted
    if dynamic_mode.is_some() {
        let body = json!({
            "dynamic": dynamic_mode.as_ref().unwrap()
        });
        let _ = client.update_index_mappings(&name, &body).await;
    }
    
    let resp = client.insert_document(&name, dat).await.unwrap();

    // If dynamic mode doesnt have any value, change it back to strict mode
    if dynamic_mode.is_none() {
        let body = json!({
            "dynamic": "strict"
        });
        let _ = client.update_index_mappings(&name, &body).await;
    }
    
    let status = resp.status_code();

    if !status.is_success() {
        return match status {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().json(json!({"error": ErrorTypes::IndexNotFound(name).to_string()})),
            StatusCode::BAD_REQUEST => HttpResponse::BadRequest().json(json!({"error": ErrorTypes::BadDataRequest.to_string()})),
            _ => HttpResponse::build(status).json(json!({"error": ErrorTypes::Unknown.to_string()})),
        }
    }

    HttpResponse::build(status).finish()
}

pub async fn get_document(data: web::Path<DocById>, query: web::Path<ReturnFields>, client: Data::<EClientTesting>) -> HttpResponse {  
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
pub async fn post_search(data: web::Json<DocumentSearch>, client: Data::<EClientTesting>) -> HttpResponse {

    let total_time_taken = std::time::Instant::now();


    let idx = data.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&data.app_id, &idx, &client).await{
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

    match document_search(&data.app_id, &idx, &body, &data.from, &data.count, &client).await {
        Ok((status, json_resp)) => {
            HttpResponse::build(status).json(json!({
                "search_took": &json_resp["took"],
                "total_took": &total_time_taken.elapsed().as_millis(),
                "data": &json_resp["hits"]["hits"],
                "total_data": &json_resp["hits"]["total"]["value"],
                "match_type": &json_resp["hits"]["total"]["relation"]
            }))
        },
        Err(err) => err,
    }
}

/// A Get method for search, also returns a list of documents from index if no query is given
pub async fn search(data: web::Path<GetDocumentSearchIndex>, query: web::Query<GetDocumentSearchQuery>, client: Data::<EClientTesting>) -> HttpResponse {

    let total_time_taken = std::time::Instant::now();

    let idx = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let fields_to_search = query.search_in.to_owned().map(|val| val.split(',').map(|x| x.trim().to_string()).collect());

    let term = if query.search_term.is_some() && query.wildcards.unwrap_or(false){
        let mut z = query.search_term.as_deref().unwrap().trim().to_string().replace(' ', "* ");
        z.push('*');
        Some(z)
    } else {
        query.search_term.clone()
    };

    let body = search_body_builder(&term, &fields_to_search, &query.return_fields);

    match document_search(&data.app_id, &idx, &body, &query.from, &query.count, &client).await {
        Ok((status, json_resp)) => {
            HttpResponse::build(status).json(json!({
                "search_took": &json_resp["took"],
                "total_took": &total_time_taken.elapsed().as_millis(),
                "data": &json_resp["hits"]["hits"],
                "total_data": &json_resp["hits"]["total"]["value"],
                "match_type": &json_resp["hits"]["total"]["relation"]
            }))
        },
        Err(x) => x,
    }
}

pub async fn update_document(data: web::Json<DocumentUpdate>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Updates the documents in shard

    let idx = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let name = index_name_builder(&data.app_id, &idx);
    
    let doc = json!({
        "doc": &data.data
    });

    let resp = client.update_document(&name, &data.document_id, &doc).await.unwrap();
    
    let status = resp.status_code();
    
    if !status.is_success() {
        let error = match status{
            StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(data.document_id.to_string()).to_string(),
            StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };
        return HttpResponse::build(status).json(json!({"error": error}));
    }

    HttpResponse::build(status).finish()
}

pub async fn delete_document(data: web::Path<DocumentDelete>, client: Data::<EClientTesting>) -> HttpResponse {  
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