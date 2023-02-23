use actix_web::{web::{self, Data}, HttpResponse, post, get, put, delete};
use crate::{EClient, routes::{index_struct::*}};


/// Creates a new dynamic index
#[post("/api/index")]
pub async fn create_new_index(data: web::Json<IndexCreate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.create_index(&data.index).await
}

/// Returns list of index if index is not provided, returns specified index if provided
#[get("/api/index")]
async fn get_all_index(index: web::Query<OptionalIndex>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.get_index(index.into_inner().index).await
}

/// Returns the mappings of an index
#[get("/api/mappings/{index}")]
async fn get_mapping(index: web::Path<RequiredIndex>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    elasticsearch_client.get_index_mappings(index.into_inner().index).await
}

/// Updates the mappings of an index
#[put("/api/mappings")]
pub async fn index_mapping_update(data: web::Json<IndexMappingUpdate>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    // Updates the mappings of an index, including its datatypes
    elasticsearch_client.update_index_mappings(&data.index, data.mappings.clone()).await
}

/// Deletes an index
#[delete("/api/index/{index}")]
pub async fn delete_index(index_to_delete: web::Path<IndexDelete>, elasticsearch_client: Data::<EClient>) -> HttpResponse {
    let dat = index_to_delete.into_inner();
    elasticsearch_client.delete_index(&dat.index).await
}