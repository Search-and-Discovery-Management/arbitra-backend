use std::{collections::HashMap, io::Read};

use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{web::{self, Data}, HttpResponse};
use ijson::IValue;
// use nanoid::nanoid;
use reqwest::StatusCode;
use serde_json::{json, Value};
use crate::{actions::EClient, handlers::{libs::{index_name_builder, search_body_builder, bulk_create}, errors::{ErrorTypes, FileErrorTypes}}, AppConfig};
use super::libs::{check_server_up_exists_app_index, document_search};
use super::structs::{document_struct::{DocumentSearchQuery, RequiredDocumentID, ReturnFields}, index_struct::RequiredIndex};

/// Document interfaces with index that is stored within the application id
/// Inserting a document with a new field syncs the fields with all other shards
/// 
/// All operations requires app_id and the index name
pub async fn create_bulk_documents(app_index: web::Path<RequiredIndex>, data: web::Json<Vec<IValue>>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse {
    println!("Route: Bulk Create Document");

    let idx = app_index.index.clone().trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client, &app_config).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    bulk_create(&app_index.app_id, &app_index.index, data.into_inner(), &client, &app_config).await
}


#[derive(MultipartForm)]
pub struct Upload {
    file: TempFile,
}

/// Inserts data from file into an existing index (Accepts json, csv, and tsv)
pub async fn create_by_file(app_index: web::Path<RequiredIndex>, f: MultipartForm<Upload>, client: web::Data<EClient>, app_config: web::Data::<AppConfig>) -> HttpResponse {
    println!("Route Create Document by File");

    match check_server_up_exists_app_index(&app_index.app_id, &app_index.index, &client, &app_config).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let file_name = f.file.file_name.clone().unwrap();
    let file_size = f.file.size;
    let mut file = f.file.file.reopen().unwrap();
    
    let file_name_split: Vec<&str> = file_name.split('.').collect();
    let extension = file_name_split.last().unwrap().to_ascii_lowercase();
    println!("file name: {:#?}", file_name);
    println!("file size: {:#?}", file_size);

    // Check the file size
    if file_size > app_config.max_input_file_size {
        return HttpResponse::PayloadTooLarge().json(json!({"error": FileErrorTypes::FileTooLarge(file_size, app_config.max_input_file_size).to_string()}))
    }

    // Check extensions, only allow json, csv, and tsv
    if extension.eq(&"json".to_string()) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let data: Result<Vec<IValue>, _> = serde_json::from_str(&contents);
        match data{
            Ok(x) => bulk_create(&app_index.app_id, &app_index.index, x, &client, &app_config).await,
            Err(_) => HttpResponse::BadRequest().json(json!({"error": FileErrorTypes::InvalidFile("json".to_string()).to_string()}))
        }
    
    } else if extension.eq(&"csv".to_string()) || extension.eq(&"tsv".to_string()) {

        let sep = if extension.eq(&"csv".to_string()){
            b','
        } else {
            b'\t'
        };

        let mut contents = csv::ReaderBuilder::new()
            .delimiter(sep)
            .from_reader(file);

        let mut data: Vec<IValue> = vec![];
        for (curr, i) in contents.deserialize().enumerate() {
            match i {
                Ok(val) => {
                    // Turn into Hashmap type before converting into value
                    let z: HashMap<String, Value> = val;
                    data.push(ijson::to_value(z).unwrap())
                },
                Err(_) => return HttpResponse::build(StatusCode::BAD_REQUEST).json(json!({"error": FileErrorTypes::InvalidLine(curr).to_string()})),
            }
        }

        return bulk_create(&app_index.app_id, &app_index.index, data, &client, &app_config).await;
    } else {
        return HttpResponse::BadRequest().json(json!({"error": FileErrorTypes::InvalidFileExtension(".json, .csv, .tsv".to_string()).to_string()}))
    }
}

pub async fn get_document(data: web::Path<RequiredDocumentID>, query: web::Path<ReturnFields>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse {  
    println!("Route: Get Document");
    // App id, index name, and document id
    // This will retrieve the shard number appended on the id then retrieve document located on that shard

    let idx = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &idx, &client, &app_config).await{
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
pub async fn post_search(app_index: web::Path<RequiredIndex>, optional_data: Option<web::Json<DocumentSearchQuery>>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse {
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
            wildcards: None,
            min_before_expansion: None
        }
    };

    let total_time_taken = std::time::Instant::now();


    let idx = app_index.index.trim().to_ascii_lowercase();
    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client, &app_config).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let fields_to_search = data.search_in.to_owned().map(|val| val.split(',').map(|x| x.trim().to_string()).collect());

    let min_before_expansion = data.min_before_expansion.unwrap_or(app_config.default_min_search_wildcard_expansion);

    let term = if data.search_term.is_some() && data.wildcards.unwrap_or(false) && !data.search_term.as_deref().eq(&Some("*")){
        let z: Vec<String> = data.search_term.as_deref().unwrap().trim().split(' ').map(|s| if s.len() >= min_before_expansion {format!("{s}*")} else {s.to_string()}).collect();
        Some(z.join(" "))
    } else {
        if !data.search_term.as_deref().eq(&Some("*")){
            data.search_term.clone()
        } else {
            None
        }
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
pub async fn search(app_index: web::Path<RequiredIndex>, data: web::Query<DocumentSearchQuery>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse {
    println!("Route: Search");

    let total_time_taken = std::time::Instant::now();

    let idx = app_index.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client, &app_config).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let fields_to_search = data.search_in.to_owned().map(|val| val.split(',').map(|x| x.trim().to_string()).collect());
    let min_before_expansion = data.min_before_expansion.unwrap_or(app_config.default_min_search_wildcard_expansion);

    let term = if data.search_term.is_some() && data.wildcards.unwrap_or(false) && !data.search_term.as_deref().eq(&Some("*")){
        let z: Vec<String> = data.search_term.as_deref().unwrap().trim().split(' ').map(|s| if s.len() >= min_before_expansion {format!("{s}*")} else {s.to_string()}).collect();
        Some(z.join(" "))
    } else {
        if !data.search_term.as_deref().eq(&Some("*")){
            data.search_term.clone()
        } else {
            None
        }
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

pub async fn update_document(app_index_doc: web::Path<RequiredDocumentID>, data: web::Json<Value>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse {  
    println!("Route: Update Document");
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Updates the documents in shard

    let idx = app_index_doc.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&app_index_doc.app_id, &idx, &client, &app_config).await{
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

pub async fn delete_document(data: web::Path<RequiredDocumentID>, client: Data::<EClient>, app_config: Data::<AppConfig>) -> HttpResponse {  
    println!("Route: Delete Document");
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Deletes the document in shard

    let idx = data.index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(&data.app_id, &idx, &client, &app_config).await{
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