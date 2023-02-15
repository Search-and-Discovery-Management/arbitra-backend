use actix_web::{get, post, web::{self, Data}, HttpResponse};
use serde::{Deserialize};
use crate::EClient;
/// Types: Post (Search), Get (Document), Get (Index)

/// Used for Post: Search
#[derive(Deserialize)]
pub struct DocumentSearch {
    index: String,
    search_term: Option<String>,
    search_in: Option<String>,
    return_fields: Option<String>,
    from: Option<i64>,
    count: Option<i64>
}

/// Used for Get: Document
#[derive(Deserialize)]
pub struct DocById{
    index: String,
    document_id: String
}

/// Used for Get: Document
#[derive(Deserialize)]
pub struct ReturnFields{
    return_fields: Option<String>
}

/// Used for Get: Index
#[derive(Deserialize)]
pub struct OptionalIndex{
    index: Option<String>
}

/// Returns documents with either match_all or multi_match
/// 
/// match_all if either "search_term" or "search_in" field is not supplied
/// 
/// multi_match if "search_term" and "search_in" is supplied
/// 
/// If "return_fields" is not supplied, defaults to returning everything
/// 
/// ```
/// Input Example:
///     json!({
///         "index": "index_name", 
///         "search_term": "term",                    // OPTIONAL
///         "search_in": "field_1,field_2,...",       // OPTIONAL
///         "return_fields": "field_1,field_2,...",   // OPTIONAL
///         "from": 123,                              // OPTIONAL
///         "count": 40                               // OPTIONAL
///     })
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Success
/// 404: Not Found
/// 400: Bad Request
/// ```
/// 
/// Success Body Example:
/// ```
/// {  
///     "data": [
///         {
///             "_id": "hID1SoYBCtUAJFK-zLaN",
///             "_index": "the_test_index",
///             "_score": 1.0,
///             "fields": {
///                 "a_vec": [
///                     "vec_data1",
///                     "vec_data2"
///                 ],
///                 "a_vec.keyword": [
///                     "vec_data1",
///                     "vec_data2"
///                 ],
///                 "field": [
///                     "field_data"
///                 ],
///                 "field.keyword": [
///                    "field_data"
///                 ],
///                 "name": [
///                     "name_data"
///                 ],
///                 "name.keyword": [
///                     "name_data"
///                 ]
///             }
///         }
///     ]
/// }
/// 
/// Error Example:
/// ```
/// {
///     "message": "not found"
/// }

#[post("/api/search")]
pub async fn search_in_index(data: web::Json<DocumentSearch>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.search_index(&data.index, data.search_term.clone(), data.search_in.clone(), data.return_fields.clone(), data.from, data.count).await
}

/// Returns a specific document
/// 
/// Optional param: fields_to_return
/// 
/// ```
/// Example:
///     127.0.0.1:8080/api/airplanes/goD1SoYBCtUAJFK-B7Z0?fields_to_return=iata_code,city
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Success
/// 404: Not Found
/// ```
#[get("/api/document/{index}/{document_id}")]
pub async fn get_document_by_id(data: web::Path<DocById>, return_fields: web::Query<ReturnFields>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = data.into_inner();
    let fields_to_return = return_fields.into_inner().return_fields;

    elasticsearch_client.get_document(dat.index, dat.document_id, fields_to_return).await
}

/// Returns list of index if index is not provided, returns specified index if provided
/// 
/// Optional param: index
/// 
/// ```
/// Example:
///     127.0.0.1:8080/api/index?index=index-name
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Success
/// 404: Not Found
/// ```
/// 
/// Success Body Example:
/// ```
/// [
///     {
///         "docs.count": "0",
///         "docs.deleted": "0",
///         "health": "green",
///         "index": "test_index",
///         "pri": "3",
///         "pri.store.size": "675b",
///         "rep": "0",
///         "status": "open",
///         "store.size": "675b",
///         "uuid": "qyX3NoR8SXOPkA0EoiDWRg"
///     }
/// ]
/// ```
#[get("/api/index")]
async fn get_all_index(index: web::Query<OptionalIndex>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.get_index(index.into_inner().index).await
}



// pub struct SearchDocument{
//     index: String
//     from: Option<i64>,
//     to: Option<i64>,

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