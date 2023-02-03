use elasticsearch::{
    Elasticsearch,
    Error,
    http::{transport::{TransportBuilder,SingleNodeConnectionPool}}, 
    indices::{IndicesExistsParts, IndicesCreateParts}, 
    IndexParts, 
    SearchParts, 
    cat::CatIndicesParts,
    DeleteParts, GetParts
};
use reqwest::{Url, StatusCode};
use serde_json::{json, Value};

pub struct EClient {
    pub req: reqwest::Client,
    pub elastic: Elasticsearch,
    pub url: Url
}

impl EClient {

    pub fn new(url: &str) -> Self {
        
        let conn_url = Url::parse(url).unwrap();

        // Elasticsearch

        let conn_pool = SingleNodeConnectionPool::new(conn_url);
        let builder = TransportBuilder::new(conn_pool);
    
        let transport = builder.build().unwrap();
        
        return Self{
            req: reqwest::Client::new(),
            elastic: Elasticsearch::new(transport),
            url: Url::parse(url).unwrap()
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
    pub async fn delete_document(&self, index: &str, id: &str) -> StatusCode{
    
        let resp = 
            self.elastic
            .delete(DeleteParts::IndexId(index, id))
            .send()
            .await
            .unwrap();
    
        return resp.status_code();
    }

    // Not Tested, Not Finished
    pub async fn find_document(&self, index: &str, search_term: Option<&str>, search_on: Option<String>, retrieve_field: Option<String>) -> Value{
        // let response = self.elastic
        //     .index(IndexParts::Index(index))
        //     .body(data)
        //     .send()
        //     .await
        //     .unwrap();

        // let term = match search_term {
        //     Some(val) => val,
        //     None => "*",
        // };

        // let search_field = match search_on {
        //     Some(val) => val,
        //     None => "*".to_string(),
        // };

        // let fields_to_retrieve = match retrieve_field {
        //     Some(val) => val.split(',').into_iter().map(|x| x.to_string()).collect(),
        //     None => vec!["*".to_string()],
        // };

        // let match_type = 
        //     match search_field {
        //         None => "match_all: ",
        //         Some(x) => "match: "
        //     };

        let body = json!({
            "query": {
                
            }
            // ,
            //     "fields": fields_to_retrieve
            // ,
        });

        println!("{:#?}", index);
        println!("\nbody\n{:#?}\n", body);
        let resp = self.elastic
            .search(SearchParts::Index(&[index]))
            .from(0)
            .size(20)
            .body(body)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
            

        println!("{:#?}", resp);

        return resp;
    }




    pub async fn get_all_index(&self) -> Value{
        let response = self.elastic
            .cat()
            .indices(CatIndicesParts::Index(&["*"]))
            .format("json")
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await;
        return response.unwrap();
    }

   pub async fn find_document_paginated(&self, index: &str, paginate_search: Option<&str> ) -> Value{
        let resp = self.elastic
            .search(SearchParts::Index(&[index]))
            .body(json!({
                "query": {
                    "match_all" : {}
                },
            }))
            .send()
            .await
            .unwrap();

        return resp.json::<Value>().await.unwrap();
    }
}