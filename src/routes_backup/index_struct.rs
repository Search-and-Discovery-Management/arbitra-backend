use serde::Deserialize;
use serde_json::Value;

/// Used for Post: Index
#[derive(Deserialize)]
pub struct IndexCreate{
    pub index: String
}

/// Used for Get: Index
#[derive(Deserialize)]
pub struct OptionalIndex{
    pub index: Option<String>
}

/// Used for Get: Mappings
#[derive(Deserialize)]
pub struct RequiredIndex{
    pub index: String
}

/// Used for Put: Mappings
#[derive(Deserialize)]
pub struct IndexMappingUpdate {
    pub index: String,
    pub mappings: Value
}

/// Used for Delete: Index
#[derive(Deserialize)]
pub struct IndexDelete {
    pub index: String
}