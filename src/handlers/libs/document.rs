use actix_web::HttpResponse;
use ijson::IValue;
use nanoid::nanoid;
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{handlers::{errors::ErrorTypes, libs::{index_name_builder, check_server_up_exists_app_index}, structs::document_struct::BulkFailures}, actions::EClient, AppConfig};

// Convert to StatusCode, Vec<Value>?
pub async fn bulk_create(app_id: &str, index: &str, data: &[IValue], client: &EClient, app_config: &AppConfig) -> HttpResponse{
    let idx = index.trim().to_ascii_lowercase();

    match check_server_up_exists_app_index(app_id, &idx, client, app_config).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let name = index_name_builder(app_id, &idx);

    let num_of_indexes = client.cat_get_index(Some(format!("{name}.*"))).await.unwrap().json::<Vec<IValue>>().await.unwrap().len();

    let mut shard_numbers: Vec<usize> = vec![];
    let mut ids: Vec<String> = vec![];

    let rng = fastrand::Rng::new();
    for _ in 0..data.len() {
        shard_numbers.push(rng.usize(0..num_of_indexes));
        ids.push(nanoid!(26));
    }

    let resp = client.bulk_create_documents(&name, data, &ids, &shard_numbers).await.unwrap();

    let mut status = resp.status_code();
    let json: Value = resp.json::<Value>().await.unwrap();

    let mut failures: Vec<BulkFailures> = vec![];
    if json["errors"].as_bool().unwrap() {
        for (loc, val) in json["items"].as_array().unwrap().iter().enumerate(){
            if !val["create"]["error"].is_null(){
                failures.push(
                    BulkFailures {
                        document_number: loc,
                        error: val["create"]["error"]["reason"].as_str().unwrap().to_string(),
                        status: val["create"]["status"].as_i64().unwrap()
                    }
                );
            }
        }
    }
    
    if json["errors"].as_bool().unwrap() {
        status = StatusCode::MULTI_STATUS;
    }

    // Force elastic to immediately index the new documents
    let body = search_body_builder(&Some("".to_string()), &Some(vec!["".to_string()]), &Some("".to_string()));
    let z = document_search(app_id, index, &body, &Some(0), &Some(0), true, client).await;
    println!("{}", z.unwrap().0);
    
    HttpResponse::build(status).json(serde_json::json!({
        "error_count": failures.len(),
        "has_errors": json["errors"].as_bool().unwrap(),
        "errors": failures
    }))
}

/// Gets a single document
/// 
/// OK: Returns statuscode and document
/// 
/// Errors: Document Not Found, or Unknown
pub async fn get_document(index: &str, document_id: &str, retrieve_fields: &Option<String>, client: &EClient) -> Result<(StatusCode, Value), (StatusCode, ErrorTypes)>{
    let resp = client.get_document(index, document_id, retrieve_fields).await.unwrap();

    let status_code = resp.status_code();
    
    if !status_code.is_success() {
        let error = match status_code{
            StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(document_id.to_string()),
            _ => ErrorTypes::Unknown
        };
        return Err((status_code, error));
    }

    let json_resp = resp.json::<Value>().await.unwrap();

    Ok((status_code, json_resp))
}

/// Searches an index
/// 
/// OK: Returns status, vec list of documents returned
/// 
/// Errors: Returns IndexNotFound, BadDataRequest, or Unknown
pub async fn document_search(app_id: &str, index: &str, body: &Value, from: &Option<i64>, count: &Option<i64>, partitioned: bool, client: &EClient) -> Result<(StatusCode, Value), HttpResponse> {

    let mut name = index_name_builder(app_id, index);
    if partitioned {
        name = format!("{name}.*");
    }
    
    let time_now = std::time::Instant::now();
    let resp = client.search_index(&name, body, from, count).await.unwrap();
    println!("Initial Request elapsed: {:#?}ms", time_now.elapsed().as_millis());

    let status = resp.status_code();

    if !status.is_success() {
        let error = match status {
            StatusCode::NOT_FOUND => ErrorTypes::IndexNotFound(index.to_owned()).to_string(),
            StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };

        return Err(HttpResponse::build(status).json(json!({"error": error})));
    };

    let receive = std::time::Instant::now();
    let json_resp = resp.json::<Value>().await.unwrap();
    println!("Body response and conversion elapsed {:#?}ms", receive.elapsed().as_millis());
    
    Ok((status, json_resp))
}

/// Returns a generated search body
pub fn search_body_builder(search_term: &Option<String>, search_in: &Option<Vec<String>>, retrieve_field: &Option<String>) -> Value {
    let fields_to_search = search_in.to_owned().unwrap_or(vec!["*".to_string()]);


    let fields_to_return = match retrieve_field {
        Some(val) => val.split(',').map(|x| x.trim().to_string()).collect(),
        None => vec!["*".to_string()],
    };

    
    // Returns everything
    let mut body = json!({
        "_source": {
            "includes": fields_to_return
        },
        "query": {
            "match_all": {} 
        },
    });
    
    // if search term exists
    if let Some(term) = search_term {
        body = json!({
            "_source": {
                "includes": fields_to_return
            },
            "query": {
                    "query_string": {
                        "query": term,
                        "type": "cross_fields",
                        "fields": fields_to_search,
                        "minimum_should_match": "75%"
                    }
                }
            })
    }
    body
}
