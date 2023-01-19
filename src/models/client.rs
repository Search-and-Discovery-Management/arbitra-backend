use elasticsearch::{
    Elasticsearch,
    Error,
    http::{transport::{TransportBuilder,SingleNodeConnectionPool}}, 
    indices::{IndicesExistsParts, IndicesCreateParts}, IndexParts, SearchParts
};
use reqwest::{Url, StatusCode};
use serde_json::{json, Value};

pub struct EClient {
    pub elastic: Elasticsearch
}

impl EClient {
    // pub fn create_client() -> Result<Elasticsearch, Error> {
    //     let url = Url::parse("http://127.0.0.1:9201").unwrap();
    
    //     let conn_pool = SingleNodeConnectionPool::new(url);
    //     let builder = TransportBuilder::new(conn_pool);
    
    //     let transport = builder.build()?;
    //     Ok(Elasticsearch::new(transport))
    // }
    
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).unwrap();

        let conn_pool = SingleNodeConnectionPool::new(url);
        let builder = TransportBuilder::new(conn_pool);
    
        let transport = builder.build().unwrap();
        return Self{
            elastic: Elasticsearch::new(transport)
        }
    }

    pub async fn create_index(&self, index: &str) -> Result<(), Error> {
        // Check if index exists
        let exists = self.elastic
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await?;
    
        // If doesnt exist, create
        if exists.status_code() == StatusCode::NOT_FOUND {
            let response = self.elastic
                .indices()
                .create(IndicesCreateParts::Index(index))
                .body(json!(
                    {
                      "mappings": { 	
                        "dynamic":"true"
                      },
                      "settings": {
                        "index.number_of_shards": 3,
                        "index.number_of_replicas": 0,
                      }
                    }
                ))
                .send()
                .await?;
    
            if !response.status_code().is_success() {
                println!("Error found: {:#?}", response);
            }
        }
    
        Ok(())
    }

    pub async fn insert_document(&self, index: &str, data: Value) -> StatusCode{
        let response = self.elastic
            .index(IndexParts::Index(index))
            .body(data)
            .send()
            .await
            .unwrap();

        return response.status_code();
    }

    // Not Tested
    pub async fn update_document(&self, index: &str, id: &str, data: Value) -> StatusCode{
        let response = self.elastic
            .index(IndexParts::IndexId(index, id))
            .body(data)
            .send()
            .await
            .unwrap();

        return response.status_code();
    }

    // Not Tested
    // pub async fn delete_document(&self, index: &str, id: &str, data: Value) -> StatusCode{

    //     return response.status_code();
    // }

    // Not Tested, Not Finished
    pub async fn find_document(&self, index: &str, search_term: &str, search_field: Option<Vec<String>>, retrieve_field: Option<Vec<String>>) -> StatusCode{
        // let response = self.elastic
        //     .index(IndexParts::Index(index))
        //     .body(data)
        //     .send()
        //     .await
        //     .unwrap();
        let fields_to_search = match search_field {
            Some(val) => val,
            None => vec!["*".to_string()],
        };

        let fields_to_retrieve = match retrieve_field {
            Some(val) => val,
            None => vec!["*".to_string()],
        };

         let resp = self.elastic
            .search(SearchParts::Index(&[index]))
            .body(json!({
                "query": {
                    "match": {
                        "body": search_term
                    }
                },
                // "fields": [
                //     fields_to_search
                // ],
            }))
            .send()
            .await
            .unwrap();

        return resp.status_code();
    }

}