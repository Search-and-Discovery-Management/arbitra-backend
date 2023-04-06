use serde::Deserialize;
use serde_json::Value;

/// Used for Post: Index
#[derive(Deserialize)]
pub struct IndexCreate{
    pub index: String,
    pub shards: Option<usize>,
    pub replicas: Option<usize>
}


#[derive(Deserialize)]
pub struct RequiredAppID{
    pub app_id: String
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

/// Used for Delete: Index
#[derive(Deserialize)]
pub struct IndexDelete {
    pub app_id: String,
    pub index: String
}