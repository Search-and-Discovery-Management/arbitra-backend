use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Used for Post: Index
#[derive(Deserialize)]
pub struct IndexCreate{
    pub index: String,
    pub shards: Option<usize>,
    pub replicas: Option<usize>
}

/// Used for Get: Index
#[derive(Deserialize)]
pub struct OptionalIndex{
    pub index: Option<String>
}

/// Used for Get: Mappings
#[derive(Deserialize)]
pub struct RequiredIndex{
    pub app_id: String,
    pub index: String
}

/// Used for Put: Mappings
#[derive(Deserialize)]
pub struct IndexMappingUpdate {
    pub app_id: String,
    pub index: String,
    pub mappings: Value
}

/// Used for Response GET: Index
#[derive(Deserialize, Serialize)]
pub struct IndexResponse {
    pub index: String,
    #[serde(rename(deserialize = "docs.count"))]
    pub docs_count: String,
    #[serde(rename(deserialize = "docs.deleted"))]
    pub docs_deleted: String
}