use actix_web::{web, HttpResponse};
use actix_web::{get, post, web::{Data}};
use serde_json::{Value};
use crate::EClient;
use crate::routes::{required_check_string, optional_check_string, optional_check_number};
use serde::{Deserialize};
/// Types: Post (Search), Get (Document), Get (Index)

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
///         "from": 123,
///         "count": 40
///     }
/// ```
#[post("/api/search")]
pub async fn search_in_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> HttpResponse {

    // let index = data.get("index");

    let idx = match required_check_string(data.get("index"), "index"){
        Ok(x) => x,
        Err(x) => return x
    };

    let search_term = match optional_check_string(data.get("search_term")){
        Some(x) => Some(x),
        None => None
    };

    let fields_to_search = match optional_check_string(data.get("search_in")){
        Some(x) => Some(x),
        None => None
    };

    let fields_to_return = match optional_check_string(data.get("return_fields")){
        Some(x) => Some(x),
        None => None
    };


    let from = match optional_check_number(data.get("from")){
        Some(x) => x,
        None => 0
    }; 

    let count = match optional_check_number(data.get("count")){
        Some(x) => x,
        None => 20
    }; 

    elasticsearch_client.search_index(&idx, search_term, fields_to_search, fields_to_return, from, count).await
}

// TODO: Turn into get with params instead of body, Unfinished

// pub struct SearchDocument{
//     index: Option<String>
//     from: i64,
//     to: i64,

// }

// #[get("/api/search")]
// pub async fn search_in_index_get(data: web::Json<SearchDocument>, elasticsearch_client: Data::<EClient>) -> HttpResponse {



//     let idx = match required_check_string(data.get("index"), "index"){
//         Ok(x) => x,
//         Err(x) => return x
//     };

//     let search_term = match optional_check_string(data.get("search_term")){
//         Some(x) => Some(x),
//         None => None
//     };

//     let fields_to_search = match optional_check_string(data.get("search_in")){
//         Some(x) => Some(x),
//         None => None
//     };

//     let fields_to_return = match optional_check_string(data.get("return_fields")){
//         Some(x) => Some(x),
//         None => None
//     };


//     let from = match optional_check_number(data.get("from")){
//         Some(x) => x,
//         None => 0
//     }; 

//     let count = match optional_check_number(data.get("count")){
//         Some(x) => x,
//         None => 20
//     }; 

//     elasticsearch_client.search_index(&idx, search_term, fields_to_search, fields_to_return, from, count).await
// }

/// Gets a document by its id
/// 
/// Requires index and document id, with return fields as optional
// #[get("/api/document")]
// pub async fn get_document_by_id(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> HttpResponse {

//     let idx = match required_check_string(data.get("index"), "index"){
//         Ok(x) => x,
//         Err(x) => return x
//     };

//     let document_id = match required_check_string(data.get("document_id"), "document id"){
//         Ok(x) => x,
//         Err(x) => return x
//     };

//     let fields_to_return = match optional_check_string(data.get("return_fields")){
//         Some(x) => Some(x),
//         None => None
//     };

//     elasticsearch_client.get_document(idx, document_id, fields_to_return).await
// }

// /// Returns list of index if index is not provided, returns specified index if provided
// #[get("/api/index")]
// async fn get_all_index(index: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
//     // if exists: return a list of index

//     let idx = match optional_check_string(index.get("index")){
//         Some(x) => Some(x),
//         None => None
//     };

//     elasticsearch_client.get_index(idx).await
// }

#[derive(Deserialize)]
pub struct DocById{
    idx: String,
    doc_id: String
}

#[derive(Deserialize)]
pub struct ReturnFields{
    fields_to_return: Option<String>
}

#[get("/api/document/{idx}/{doc_id}")]
pub async fn get_document_by_id(data: web::Path<DocById>, return_fields: web::Query<ReturnFields>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = data.into_inner();
    let fields_to_return = return_fields.into_inner().fields_to_return;

    elasticsearch_client.get_document(dat.idx, dat.doc_id, fields_to_return).await
}

#[derive(Deserialize)]
pub struct OptionalIndex{
    index: Option<String>
}

/// Returns list of index if index is not provided, returns specified index if provided
/// 
/// Optional param: index
/// 
/// ```
/// Example:
///     127.0.0.1:8080/api/index?index=index-name
/// ```
#[get("/api/index")]
async fn get_all_index(index: web::Query<OptionalIndex>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.get_index(index.into_inner().index).await
}