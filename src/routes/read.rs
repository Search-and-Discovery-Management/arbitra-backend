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

/// Used for Get: Mappings
#[derive(Deserialize)]
pub struct RequiredIndex{
    index: String
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

/// Returns the mappings of an index
#[get("/api/mappings/{index}")]
async fn get_mapping(index: web::Path<RequiredIndex>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.get_index_mappings(index.into_inner().index).await
}