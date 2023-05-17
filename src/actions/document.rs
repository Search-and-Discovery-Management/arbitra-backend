use std::iter::zip;

use elasticsearch::{IndexParts, UpdateParts, SearchParts, GetSourceParts, DeleteParts, http::response::Response, Error, BulkOperation, BulkParts, BulkOperations};
use ijson::IValue;
use serde_json::{Value};

use super::EClient;

impl EClient {
    /// Inserts a new document into index
    pub async fn insert_document(&self, index: &str, data: &Value) -> Result<Response, Error>{
        self.elastic
            .index(IndexParts::Index(index))
            .body(data)
            .send()
            .await
    }

    pub async fn bulk_create_documents(&self, index: &str, data: &[IValue], ids: &[String], shard_number: &[usize]) -> Result<Response, Error> {

        let mut body: Vec<BulkOperation<_>> = vec![];

        for ((val, id), shard) in zip(zip(data.iter(), ids), shard_number){
            body.push(BulkOperation::create(format!("{id}.{shard}"), val).index(format!("{index}.{shard}")).into())
        }

        self.elastic
            .bulk(BulkParts::Index(index))
            .body(body)
            .send()
            .await
    }

    /// Finds document in index
    pub async fn search_index(&self, index: &str, body: &Value, from: &Option<i64>, count: &Option<i64>) -> Result<Response, Error>{

        let from = from.unwrap_or(0);
        let count = count.unwrap_or(20);
        // let header = HeaderName::from_static("accept-encoding");
        // let value = HeaderValue::from_str("gzip, deflate, br").unwrap();

        self.elastic
            .search(SearchParts::Index(&[index]))
            .from(from)
            .size(count)
            .body(body)
            // .header(header, value)
            .send()
            .await
    }

    /// Returns a single document
    pub async fn get_document(&self, index: &str, doc_id: &str, retrieve_fields: &Option<String>) -> Result<Response, Error>{
        
        let fields_to_return = retrieve_fields.as_deref().unwrap_or("*");
        // let resp = client.elastic
        //     .get(GetParts::IndexId(index, document_id))
        //     .send()
        //     .await
        //     .unwrap();

        self.elastic
            .get_source(GetSourceParts::IndexId(index, doc_id))
            ._source_includes(&[fields_to_return])
            .send()
            .await
    }
    
    /// Updates existing document on an index
    pub async fn update_document(&self, index: &str, document_id: &str, data: &Value) -> Result<Response, Error> {//(StatusCode, Value){
        self.elastic
            .update(UpdateParts::IndexId(index, document_id))
            .body(data)
            .send()
            .await
    }

    // Testing
    pub async fn bulk_update_documents(&self, index: &str, data: &[Value], ids: &[String], shard_number: &[usize]) -> Result<Response, Error> {
        // let body: Vec<BulkOperation<_>> = data
        //     .iter()
        //     .map(|p| {
        //         BulkOperation::update(, p).into()
        //     })
        //     .collect();

        let mut body: Vec<BulkOperation<_>> = vec![];

        for ((val, id), shard) in zip(zip(data.iter(), ids), shard_number){
            body.push(BulkOperation::update(id, val).index(format!("{index}.{shard}")).into())
        }

        self.elastic
            .bulk(BulkParts::Index(index))
            .body(body)
            .send()
            .await
    }

    pub async fn bulk(&self, operations: BulkOperations) -> Result<Response, Error> {
        // let body: Vec<BulkOperation<_>> = data
        //     .iter()
        //     .map(|p| {
        //         BulkOperation::update(, p).into()
        //     })
        //     .collect();

        self.elastic
            .bulk(BulkParts::None)
            .body(vec![operations])
            .send()
            .await
    }


    /// Deletes document on an index
    pub async fn delete_document(&self, index: &str, document_id: &str) -> Result<Response, Error>{
        self.elastic
            .delete(DeleteParts::IndexId(index, document_id))
            .send()
            .await
    }
}