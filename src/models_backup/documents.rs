use actix_web::HttpResponse;
use elasticsearch::{IndexParts, UpdateParts, SearchParts, GetSourceParts, DeleteParts};
use reqwest::StatusCode;
use serde_json::{Value, json};

use super::{EClient, ErrorTypes, helpers::{index_exists_check_or_down}};

impl EClient {
    /// Inserts a new document into index
    pub async fn insert_document(&self, index: &str, data: Value, dynamic_mode: Option<String>) -> HttpResponse{

        match index_exists_check_or_down(&self.elastic, index).await{
            Ok(()) => (),
            Err(x) => return x
        };
  
        match dynamic_mode {
            Some(mode) => {
                let set_dynamic = json!({
                    "dynamic": mode
                });
        
                self.update_index_mappings(index, set_dynamic).await;
            },
            None => (),
        }

        let resp = self.elastic
            .index(IndexParts::Index(index))
            .body(data)
            .send()
            .await
            .unwrap();

        let status_code = resp.status_code();

        if !status_code.is_success() {
            let error = match status_code{
                StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
                _ => ErrorTypes::Unknown.to_string()
            };
            return HttpResponse::build(status_code).json(json!({"error": error}));
        }

        let set_dynamic = json!({
            "dynamic": "strict"
        });
            
        self.update_index_mappings(index, set_dynamic).await;

        HttpResponse::build(status_code).finish()
    }

    /// Finds document in index
    pub async fn search_index(&self, index: &str, search_term: Option<String>, search_in: Option<String>, retrieve_field: Option<String>, from: Option<i64>, count: Option<i64>) -> HttpResponse{

        match index_exists_check_or_down(&self.elastic, index).await{
            Ok(()) => (),
            Err(x) => return x
        };

        let fields_to_search: Option<Vec<String>> = search_in.map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());

        let from = from.unwrap_or(0);
        let count = count.unwrap_or(20);

        // Gives the current page with the amount of count
        let from_page = from * count;

        let fields_to_return = match retrieve_field {
            Some(val) => val.split(',').into_iter().map(|x| x.trim().to_string()).collect(),
            None => vec!["*".to_string()],
        };

        // Returns everything
        let mut body = json!({
            "_source": false,
            "query": {
                "match_all": {} 
            },
            "fields": fields_to_return
        });

        // Some(temp_variable) = existing_variable {function(temp_variable from existing_variable)}
        if let Some(search) = search_term {
            if let Some(search_field) = fields_to_search {
                body = json!({
                    "_source": false,
                    "query": {
                        "multi_match": {
                            "query": search,
                            "fields": search_field,
                            "fuzziness": "AUTO"     
                        }
                    },
                    "fields": fields_to_return
                })
            } else {
                body = json!({
                    "_source": false,
                    "query": {
                        "multi_match": {
                            "query": search,
                            "fields": "*",
                            "fuzziness": "AUTO"     
                        }
                    },
                    "fields": fields_to_return
                });
            }
        };

        let resp = self.elastic
            .search(SearchParts::Index(&[index]))
            .from(from_page)
            .size(count)
            .body(body)
            .send()
            .await
            .unwrap();

        let status_code = resp.status_code();

        let json_resp = 
            resp.json::<Value>()
            .await
            .unwrap();

        if !status_code.is_success() {
            let error = match status_code{
                StatusCode::NOT_FOUND => ErrorTypes::IndexNotFound(index.to_string()).to_string(),
                StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
                _ => ErrorTypes::Unknown.to_string()
            };
            return HttpResponse::build(status_code).json(json!({"error": error}));
        }
        
        HttpResponse::build(status_code).json(json!({
            "took": json_resp["took"],
            "data": json_resp["hits"]["hits"],
            "total_data": json_resp["hits"]["total"]["value"],
            "match_type": json_resp["hits"]["total"]["relation"]
        }))
    }

    /// Returns a single document
    pub async fn get_document(&self, index: String, doc_id: String, retrieve_fields: Option<String>) -> HttpResponse{

        match index_exists_check_or_down(&self.elastic, &index).await{
            Ok(()) => (),
            Err(x) => return x
        };

        let fields_to_return = match retrieve_fields {
            Some(val) => val,
            None => "*".to_string(),
        };

        let resp = self.elastic
            .get_source(GetSourceParts::IndexId(&index, &doc_id))
            ._source_includes(&[&fields_to_return])
            .send()
            .await
            .unwrap();

        let status_code = resp.status_code();
        
        if !status_code.is_success() {
            let error = match status_code{
                StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(doc_id.to_string()).to_string(),
                _ => ErrorTypes::Unknown.to_string()
            };
            return HttpResponse::build(status_code).json(json!({"error": error}));
        }

        let json_resp = resp.json::<Value>().await.unwrap();


        HttpResponse::build(status_code).json(json_resp)
    }
    
    /// Updates existing document on an index
    pub async fn update_document(&self, index: &str, document_id: &str, data: Value) -> HttpResponse {//(StatusCode, Value){

        match index_exists_check_or_down(&self.elastic, index).await{
            Ok(()) => (),
            Err(x) => return x
        };

        let resp = self.elastic
            .update(UpdateParts::IndexId(index, document_id))
            .body(data)
            .send()
            .await
            .unwrap();
        
        let status_code = resp.status_code();
        
        if !status_code.is_success() {
            let error = match status_code{
                StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(document_id.to_string()).to_string(),
                StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
                _ => ErrorTypes::Unknown.to_string()
            };
            return HttpResponse::build(status_code).json(json!({"error": error}));
        }

        HttpResponse::build(status_code).finish()
    }

    /// Deletes document on an index
    pub async fn delete_document(&self, index: &str, document_id: &str) -> HttpResponse{

        match index_exists_check_or_down(&self.elastic, index).await{
            Ok(()) => (),
            Err(x) => return x
        };

        let resp = 
            self.elastic
            .delete(DeleteParts::IndexId(index, document_id))
            .send()
            .await
            .unwrap();
    
        let status_code = resp.status_code();

        if !status_code.is_success() {
            let error = match status_code{
                StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(document_id.to_string()).to_string(),
                _ => ErrorTypes::Unknown.to_string()
            };
            return HttpResponse::build(status_code).json(json!({"error": error}));
        }

        HttpResponse::build(status_code).finish()
    }
}