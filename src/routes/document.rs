use actix_web::{post, web::{self, Data}, HttpResponse, get, put, delete};
use serde_json::json;
use crate::{EClient, routes::{str_or_default_if_exists_in_vec, document_struct::*}};

/// Inserts a new document, with 3 dynamic modes: true, false, strict
/// 
/// "true" -> allow creation of new fields and partial inserts
/// 
/// "false" -> does not allow creation of new fields, only inserts new entry to existing fields with the rest lost
/// 
/// "strict" -> does not insert if it has new fields, allows partial inserts
/// 
/// Input example:
/// 
/// ```
/// json!({
///     "index": "test_index",
///     "dynamic_mode": true, // OPTIONAL
///     "data": {
///         "name": "name1",
///         "password": "password1",
///         "etc": "etc1"
///     }
/// )}
/// 
/// ```
/// 
/// Returns StatusCode: 
/// ```
/// 201: Data Created Successfully
/// 400: Bad Request 
/// 404: Not Found
/// ```
/// 
/// Does not return a body
#[post("/api/document")]
pub async fn add_data_to_index(data: web::Json<DocumentCreate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {  
    let dat = data.into_inner();
    
    let set_dynamic_mode = match dat.dynamic_mode{
        Some (x) => Some(str_or_default_if_exists_in_vec(&x, vec!["true".to_string(), "false".to_string(), "strict".to_string()], "strict")),
        None => None
    };

    elasticsearch_client.insert_document(&dat.index, dat.data, set_dynamic_mode).await
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
///     ],
///     "match_type": "eq",
///     "status": 200,
///     "took": 12,
///     "total_data": 3282
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

#[get("/api/search/{index}")]
pub async fn get_search_in_index(data: web::Path<GetDocumentSearchIndex>, query: web::Query<GetDocumentSearchQuery>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.search_index(&data.index, query.search_term.clone(), query.search_in.clone(), query.return_fields.clone(), query.from, query.count).await
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

/// Updates document on index
/// 
/// ```
/// Input Example:
///     json!({
///         "index": "index_name", 
///         "document_id": "document_id"
///         "data": {
///             "name": "username_test",
///             "password": "test_password",
///             ...
///         }
///     })
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Success
/// 400: Bad Request
/// 404: Not Found
/// ```
/// 
/// Does not return body if success
/// 
/// Example Error Body Example:
/// ```
/// {
///     "message": "not found"
/// }
/// ```
#[put("/api/document")]
pub async fn update_data_on_index(data: web::Json<DocumentUpdate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    // Update document on index

    // doc is required for updating index, read:
    // https://stackoverflow.com/questions/57564374/elasticsearch-update-gives-unknown-field-error
    let doc = json!({
        "doc": data.data.clone()
    });

    elasticsearch_client.update_document(&data.index, &data.document_id, doc).await
}

/// Deletes document in index
/// ```
/// index: index_name
/// document_id: document_id
/// ```
/// 
/// Returns StatusCode:
/// ```
/// 200: Deleted successfully
/// 404: Not Found
/// ```
/// 
/// Returns body:
/// ```
/// {
///     "message": "error_or_success"
/// }
/// ```
#[delete("/api/document/{index}/{document_id}")]
pub async fn delete_data_on_index(document_to_delete: web::Path<DocumentDelete>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = document_to_delete.into_inner();
    elasticsearch_client.delete_document(&dat.index, &dat.document_id).await
}