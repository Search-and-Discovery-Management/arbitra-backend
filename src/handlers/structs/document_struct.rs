use serde::{Deserialize, Serialize};

/// Used for GET/POST: Search
#[derive(Deserialize)]
pub struct DocumentSearchQuery {
    pub search_term: Option<String>,
    pub search_in: Option<String>,
    pub return_fields: Option<String>,
    pub from: Option<i64>,
    pub count: Option<i64>,
    pub wildcards: Option<bool>,
    // pub min_percentage_match: Option<i64>
}

/// Used for Get: Document
#[derive(Deserialize)]
pub struct ReturnFields{
    pub return_fields: Option<String>
}

/// For Bulk Errors Output
#[derive(Serialize)]
pub struct BulkFailures {
    pub document_number: usize,
    pub error: String,
    pub status: i64
}

/// Used for Delete: Document
#[derive(Deserialize)]
pub struct RequiredDocumentID {
    pub app_id: String,
    pub index: String,
    pub document_id: String
}