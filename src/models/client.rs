use actix_web::HttpResponse;
use elasticsearch::{
    Elasticsearch,
    http::{transport::{TransportBuilder,SingleNodeConnectionPool}}, 
    indices::{IndicesExistsParts, IndicesCreateParts, IndicesPutMappingParts}, 
    IndexParts, 
    SearchParts, 
    cat::CatIndicesParts,
    DeleteParts, UpdateParts, GetSourceParts
};
use reqwest::{Url, StatusCode};
use serde_json::{json, Value};

pub struct EClient {
    pub req: reqwest::Client,
    pub elastic: Elasticsearch,
    pub url: Url
}

impl EClient {

    /// Creates a new instance of EClient
    /// 
    /// Connects to an instance of ElasticSearch server
    /// 
    /// Url: IP with Port (ex: "http://192.168.0.1:9200")
    pub fn new(url: &str) -> Self {
        
        let conn_url = Url::parse(url).unwrap();

        // Elasticsearch

        let conn_pool = SingleNodeConnectionPool::new(conn_url);
        let builder = TransportBuilder::new(conn_pool);
    
        let transport = builder.build().unwrap();
        
        Self{
            req: reqwest::Client::new(),
            elastic: Elasticsearch::new(transport),
            url: Url::parse(url).unwrap()
        }
    }


    /// Creates a new index
    /// 
    /// index: Index Name
    pub async fn create_index(&self, index: &str) -> HttpResponse{
        // Check if index exists
        let exists = self.elastic
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await
            .unwrap();

        if exists.status_code().is_success() {
            return HttpResponse::Conflict().finish();
        }

        if exists.status_code() == StatusCode::NOT_FOUND {
            let resp = self.elastic
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
                .await
                .unwrap();
    
            if resp.status_code() == StatusCode::OK {
                return HttpResponse::Created().finish();
            }
            return HttpResponse::build(resp.status_code()).finish();
        }
    
        HttpResponse::build(exists.status_code()).finish()
    }

    /// Inserts a new document into index
    /// 
    /// index: Index Name (ex: airplanes)
    /// 
    /// Data: Data to insert, data type is serde_json::Value
    /// ```
    /// Example:
    ///     json!({
    ///         "name": "test_name",
    ///         "password": "test_password",
    ///         "email": "test_email@example.com"
    ///     })
    /// ```
    /// Returns status code wheter or not it succeed, such as 404 if index not found
    /// 
    /// Dynamic mode: Consists of "true", "false", "strict"
    pub async fn insert_document(&self, index: &str, data: Value, dynamic_mode: Option<String>) -> HttpResponse{

        let exists = self.elastic
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await
            .unwrap();
    
            if !exists.status_code().is_success() {
                return HttpResponse::build(exists.status_code()).finish();
            }
            
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

        let set_dynamic = json!({
            "dynamic": "strict"
        });
            
        self.update_index_mappings(index, set_dynamic).await;

        HttpResponse::build(resp.status_code()).finish()
    }

    pub async fn update_index_mappings(&self, index: &str, mappings: Value) -> HttpResponse{

        let exists = self.elastic
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await
            .unwrap();
    
        if !exists.status_code().is_success() {
            return HttpResponse::build(exists.status_code()).finish();
        }

        let resp = self.elastic
            .indices()
            .put_mapping(IndicesPutMappingParts::Index(&[index]))
            .body(mappings)
            .send()
            .await
            .unwrap();

        let status_code = resp.status_code();

        // println!("{:#?}", resp);
        // println!("{:#?}", resp.json::<Value>().await.unwrap());

        HttpResponse::build(status_code).finish()
    }

    /// Updates existing document on an index
    /// 
    /// index: Index name (ex: airplanes)
    /// 
    /// document_id: Document id (ex: BnqR-YUBLRm6224CUtK7)
    /// 
    /// Data: Data to insert, data type is serde_json::Value
    /// ```
    /// Example: 
    ///     json!{(
    ///         "doc": {
    ///             "name": "test_name",
    ///             "password": "test_password",
    ///             "email": "test_email@example.com"
    ///         }
    ///     })
    /// ```
    /// Returns a json consisting of 
    /// ```
    /// Example:
    /// 
    /// Error:
    ///     json!{
    ///         "message": "Not Found",
    ///     }
    /// 
    pub async fn update_document(&self, index: &str, document_id: &str, data: Value) -> HttpResponse {//(StatusCode, Value){

        let resp = self.elastic
            .update(UpdateParts::IndexId(index, document_id))
            .body(data)
            .send()
            .await
            .unwrap();

        let status_code = resp.status_code();

        let json_resp = resp.json::<Value>().await.unwrap();
        
        let message = if !status_code.is_success() {
            &json_resp["error"]["reason"]
        } else {
            return HttpResponse::build(status_code).finish();
        };

        HttpResponse::build(status_code).json(json!({
            "message": message
        }))
        // println!("{:#?}", json_resp);


        // (status_code, json!({
        //     "message": json_resp["error"]["reason"]
        // }))
    }

    /// Deletes document on an index
    /// 
    /// index: Index name (ex: airplanes)
    /// 
    /// id: Document id (ex: BnqR-YUBLRm6224CUtK7)
    /// 
    /// Returns status code whether or not it succeed, such as 200 if succeed, 404 if either index or document is not found
    pub async fn delete_document(&self, index: &str, document_id: &str) -> HttpResponse{

        let resp = 
            self.elastic
            .delete(DeleteParts::IndexId(index, document_id))
            .send()
            .await
            .unwrap();
    
        let status_code = resp.status_code();

        let json_resp =  resp
            .json::<Value>()
            .await
            .unwrap();

        // if !status_code.is_success(){
        //     return json!({
        //         "status_code": status_code.as_u16(),
        //         "error_message": json_resp["result"]
        //     });
        // }

        HttpResponse::build(status_code).json(
            json!({
                "message": json_resp["result"]
            })
        )

        // (status_code, json!({
        //     "message": json_resp["result"]
        // }))
    }

    /// Finds document in index
    /// 
    /// index: Index name
    /// 
    /// search_term: String to search, ex: abcd
    /// 
    /// search_on: Fields to search, ex: field_1,field_2,field_3
    /// 
    /// retrieve_fields: Fields to return, ex: field_1,field_2,field_3
    /// 
    /// from: i64, Offset of documents to return from "from", ex: 20
    /// 
    /// count: Number of documents to return, ex: 10
    /// 
    /// Example Return:
    /// 
    /// ```
    /// 
    /// {
    ///     status_code: "200",
    ///     took: "1"
    ///     data: {
    ///     ...
    ///     },
    ///     total_data: 1234,
    ///     match_type: "eq"
    /// 
    /// }
    /// ```
    pub async fn search_index(&self, index: &str, search_term: Option<String>, search_on: Option<String>, retrieve_field: Option<String>, from: Option<i64>, count: Option<i64>) -> HttpResponse{

        let fields_to_search: Option<Vec<String>> = search_on.map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());

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
            return HttpResponse::build(status_code).json(json!({
                "message": json_resp["error"]["reason"]
            }));
        }
        
        HttpResponse::build(status_code).json(json!({
            "status": status_code.as_u16(),
            "took": json_resp["took"],
            "data": json_resp["hits"]["hits"],
            "total_data": json_resp["hits"]["total"]["value"],
            "match_type": json_resp["hits"]["total"]["relation"]
        }))
    }


    /// Returns either a list of index if index is not supplied, or the specified index
    pub async fn get_index(&self, index: Option<String>) -> HttpResponse{

        let idx = match index {
            Some(x) => x,
            None => "*".to_string()
        };

        let resp = self.elastic
            .cat()
            .indices(CatIndicesParts::Index(&[&idx]))
            .format("json")
            .send()
            .await
            .unwrap();

        let status_code = resp.status_code();
        
        let json_resp = resp.json::<Value>().await.unwrap();


        if !status_code.is_success() {
            return HttpResponse::build(status_code).json(json!({
                "message": json_resp["error"]["reason"]
            }));
        }

        HttpResponse::build(status_code).json(json_resp)
    }

    pub async fn get_document(&self, index: String, doc_id: String, retrieve_fields: Option<String>) -> HttpResponse{

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
        
        let json_resp = resp.json::<Value>().await.unwrap();


        if !status_code.is_success() {
            return HttpResponse::build(status_code).json(json!({
                // "status_code": status_code.as_u16(),
                "message": json_resp["error"]["reason"]
            }));
        }

        HttpResponse::build(status_code).json(json_resp)
    }

//    pub async fn find_document_paginated(&self, index: &str, paginate_search: Option<&str> ) -> Value{
//         let resp = self.elastic
//             .search(SearchParts::Index(&[index]))
//             .body(json!({
//                 "query": {
//                     "match_all" : {}
//                 },
//             }))
//             .send()
//             .await
//             .unwrap();

//         return resp.json::<Value>().await.unwrap();
//     }
}