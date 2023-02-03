use actix_web::web;
use actix_web::{get, web::{Data}, Responder, Result};
use serde_json::Value;
use crate::EClient;
// mod error;
// use crate::errors::*;

// JSON Data Format For Get:
// ```
//     {
//         "index": index_name
//         "search_term": ABC
//         "search_in": [
//             name,
//             password,
//             ...
//         ]
//         "return_fields": [
//             name,
//             password,
//             ...
//         ]
//     }
// ```
// From serde_json value, extract: 
// let x = var.get("str");


/// Returns documents with either match_all or multi_match
/// 
/// match_all if either "search_term" or "search_in" field is not supplied
/// 
/// multi_match if "search_term" and "search_in" is supplied
/// 
/// If "return_fields" is not supplied, defaults to returning everything
/// 
/// ```
/// Input:
///     {
///         "index": "index_name",
///         "search_term": "term",
///         "search_in": "field_1,field_2,...",
///         "return_fields": "field_1,field_2,...",
///         "from": 123, // TODO
///         "count": 40 // TODO
///     }
/// ```
#[get("/api/search_documents")]
pub async fn search_in_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    let index = data.get("index");

    let search_term = data.get("search_term").map(|val| val.as_str().unwrap());

    let fields_to_search = data.get("search_in").map(|val| val.as_str().unwrap());

    let fields_to_return = data.get("return_fields").map(|val| val.as_str().unwrap());

    let resp = elasticsearch_client.find_document(index.unwrap().as_str().unwrap(), search_term, fields_to_search, fields_to_return, 0, 20).await;

    Ok(web::Json(resp))
}

/// Returns list of index
/// 
/// Does not accept json input
#[get("/api/get_index_list")]
async fn get_all_index(elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {
    // if exists: return a list of index
    let resp = elasticsearch_client.get_all_index().await;

    Ok(web::Json(resp))
}