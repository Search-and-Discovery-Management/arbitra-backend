use actix_web::{web::{self, Data}, HttpResponse};
use reqwest::StatusCode;
use serde_json::{json, Value};
use crate::{actions::EClientTesting, handlers::{libs::{index_name_builder, search_body_builder}, errors::ErrorTypes}};
use super::{document_struct::{DocumentCreate, GetDocumentSearchIndex, GetDocumentSearchQuery, DocumentSearch, DocumentUpdate, DocumentDelete, DocById, ReturnFields}, libs::{get_mapping_keys, check_server_up_exists_app_index}};

/// Document interfaces with index that is stored within the application id
/// Inserting a document with a new field syncs the fields with all other shards
/// 
/// All operations requires app_id and the index name
/// 

// Temp _ because models and routes having same name

pub async fn _create_document(data: web::Json<DocumentCreate>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    // match index_exists(&data.app_id, &data.index, &client).await{
    //     Ok(_) => (),
    //     Err((x, y, _)) => return HttpResponse::build(x).json(json!({"error": y.to_string()}))
    // };
    
    // Check app exists -> check index exists
    // -> Get document

    // Creates a new document by getting application id, index name, check if document has new field, if yes, check dynamic mode
    // if true, update the entire index shards to accomodate the new field, then insert

    // TODO: Change ID to have appended shard number
    // Solutions: Either use fingerprint or use bulk api, the latter being most likely since it allows direct insert

    // Inserts document into index -> Checks if app has index
    // Checks if index exists
    // Insert
    
    let name = index_name_builder(&data.app_id, &data.index);
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

pub async fn _get_document(data: web::Path<DocById>, query: web::Path<ReturnFields>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // App id, index name, and document id
    // This will retrieve the shard number appended on the id

    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };

    let name = &index_name_builder(&data.app_id, &data.index);

    let resp = client.get_document(name, &data.document_id, query.return_fields.to_owned()).await.unwrap();

    let status_code = resp.status_code();
    
    if !status_code.is_success() {
        let error = match status_code{
            StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(data.document_id.to_string()).to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };
        return HttpResponse::build(status_code).json(json!({"error": error}));
    }

    let json_resp = resp.json::<Value>().await.unwrap();

    HttpResponse::build(status_code).json(json_resp)
}

/// Returns a list of documents from index, post method
pub async fn _post_search(data: web::Json<DocumentSearch>, client: Data::<EClientTesting>) -> HttpResponse {
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }

    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };

    let name = index_name_builder(&data.app_id, &data.index);
    // let keys: HashMap<String, Vec<String>> = maps.json::<HashMap<String, Vec<String>>>().await.unwrap();
    // for (i, val) in keys {
    //     println!("string: {i}, val: {:#?}", val);
    // }
    
    // TODO: Default search_in into a type of searchableAttributes which defaults its search to all fields with searchableAttributes when nothing is supplied 
    // let fields_to_search: Option<Vec<String>>;
    let fields_to_search = data.search_in.to_owned().map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());
    // if data.search_in.is_some(){
    // } else {
    //     fields_to_search = Some(get_mapping_keys(&name, &client).await)
    // }
    // let fields_to_search: Option<Vec<String>> = data.search_in.to_owned().map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());

    // println!("{:#?}", fields_to_search);

    let body = search_body_builder(data.search_term.to_owned(), fields_to_search, data.return_fields.to_owned(), Some("2".to_string()));

    let resp = client.search_index(&name, &body, data.from.to_owned(), data.count.to_owned()).await.unwrap();

    let status = resp.status_code();

    if !status.is_success() {
        let error = match status {
            StatusCode::NOT_FOUND => ErrorTypes::IndexNotFound(data.index.to_owned()).to_string(),
            StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };

        return HttpResponse::build(status).json(json!({"error": error}));
    };

    let json_resp = resp.json::<Value>().await.unwrap();
    // println!("{:#?}", json_resp);
    HttpResponse::build(status).json(json!({
        "took": json_resp["took"],
        "data": json_resp["hits"]["hits"],
        "total_data": json_resp["hits"]["total"]["value"],
        "match_type": json_resp["hits"]["total"]["relation"]
    }))
}

/// Returns a list of documents from index
pub async fn _search(data: web::Path<GetDocumentSearchIndex>, query: web::Query<GetDocumentSearchQuery>, client: Data::<EClientTesting>) -> HttpResponse {
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }

    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };

    let name = index_name_builder(&data.app_id, &data.index);

    let fields_to_search: Option<Vec<String>>;
    if query.search_in.is_some(){
        fields_to_search = query.search_in.to_owned().map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());
    } else {
        fields_to_search = Some(get_mapping_keys(&name, &client).await)
    }


    let body = search_body_builder(query.search_term.to_owned(), fields_to_search, query.return_fields.to_owned(), Some("2".to_string()));

    let resp = client.search_index(&name, &body, query.from, query.count).await.unwrap();

    let status = resp.status_code();

    if !status.is_success() {
        let error = match status {
            StatusCode::NOT_FOUND => ErrorTypes::IndexNotFound(data.index.to_owned()).to_string(),
            StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };

        return HttpResponse::build(status).json(json!({"error": error}));
    };

    let json_resp = resp.json::<Value>().await.unwrap();

    // HttpResponse::build(status).json(json_resp)
    HttpResponse::build(status).json(json!({
        "took": json_resp["took"],
        "data": json_resp["hits"]["hits"],
        "total_data": json_resp["hits"]["total"]["value"],
        "match_type": json_resp["hits"]["total"]["relation"]
    }))
}

pub async fn _update_document(data: web::Json<DocumentUpdate>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Updates the documents in shard

    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };
    
    let name = index_name_builder(&data.app_id, &data.index);
    
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

pub async fn _delete_document(data: web::Path<DocumentDelete>, client: Data::<EClientTesting>) -> HttpResponse {  
    // if !is_server_up(&client).await { return HttpResponse::ServiceUnavailable().json(json!({"error": ErrorTypes::ServerDown.to_string()})) }
    // Deletes the document in shard

    match check_server_up_exists_app_index(&data.app_id, &data.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()}))
    };

    let name = index_name_builder(&data.app_id, &data.index);

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