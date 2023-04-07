use serde::Deserialize;


/// Used for Post: Search
// #[derive(Deserialize)]
// pub struct DocumentSearch {
//     pub search_term: Option<String>,
//     pub search_in: Option<String>,
//     pub return_fields: Option<String>,
//     pub from: Option<i64>,
//     pub count: Option<i64>,
//     pub wildcards: Option<bool>,
//     // pub min_percentage_match: Option<i64>
// }

/// Used for Get: Search
// #[derive(Deserialize)]
// pub struct GetDocumentSearchIndex {
//     pub app_id: String,
//     pub index: String
// }

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

/// Used for Post: Document_multiple -- UNUSED
// #[derive(Deserialize)]
// pub struct DocumentCreateBulk{
//     pub app_id: String,
//     pub index: String,
//     pub data: Vec<Value>,
//     pub dynamic_mode: Option<String>
// }

/// Used for Put: Document
// #[derive(Deserialize)]
// pub struct DocumentUpdate {
//     pub app_id: String,
//     pub index: String,
//     pub document_id: String,
//     pub data: Value
// }

/// Used for Delete: Document
#[derive(Deserialize)]
pub struct RequiredDocumentID {
    pub app_id: String,
    pub index: String,
    pub document_id: String
}