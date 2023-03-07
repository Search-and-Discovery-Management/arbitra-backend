use elasticsearch::{indices::{IndicesCreateParts, IndicesPutMappingParts, IndicesGetMappingParts, IndicesDeleteParts, IndicesExistsParts}, cat::CatIndicesParts, http::response::Response, Error};
use serde_json::Value;

use super::{EClientTesting};



impl EClientTesting{
    /// Creates a new index
    pub async fn create_index(&self, body: Value, index: &str) -> Result<Response, Error>{
        self.elastic
            .indices()
            .create(IndicesCreateParts::Index(index))
            .body(body)
            .send()
            .await
    }

    // Updates the mappings of an index
    pub async fn update_index_mappings(&self, index: &str, mappings: Value) -> Result<Response, Error>{
        self.elastic
            .indices()
            .put_mapping(IndicesPutMappingParts::Index(&[index]))
            .body(mappings)
            .send()
            .await
    }

    /// Returns either a list of index if index is not supplied, or the specified index
    pub async fn get_index(&self, index: Option<String>) -> Result<Response, Error>{
        self.elastic
            .cat()
            .indices(CatIndicesParts::Index(&[&index.unwrap_or("*".to_string())]))
            .format("json")
            .send()
            .await
    }

    /// Checks if index exists
    pub async fn check_index(&self, index: &str) -> Result<Response, Error>{
        // Check if index exists
        self.elastic
            .indices()
            .exists(IndicesExistsParts::Index(&[&index]))
            .send()
            .await
    }

    /// Returns the mappings of an index
    pub async fn get_index_mappings(&self, index: &str) -> Result<Response, Error>{
        self.elastic
            .indices()
            .get_mapping(IndicesGetMappingParts::Index(&[&index]))
            .send()
            .await
    }
    
    /// Deletes an index
    pub async fn delete_index(&self, index: &str) -> Result<Response, Error>{
        self.elastic
            .indices()
            .delete(IndicesDeleteParts::Index(&[index]))
            .send()
            .await
    }

}