use elasticsearch::{
    Elasticsearch,
    http::{transport::{TransportBuilder,SingleNodeConnectionPool}}, 
    indices::{IndicesExistsParts, IndicesCreateParts}, 
    IndexParts, 
    SearchParts, 
    cat::CatIndicesParts,
    DeleteParts, UpdateParts
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
    /// 
    /// TODO: Change return type (StatusCode?)
    pub async fn create_index(&self, index: &str) -> StatusCode{
        // Check if index exists
        let exists = self.elastic
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await
            .unwrap();
    
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
                .await
                .unwrap();
    
            return response.status_code();
        }
    
        exists.status_code()
    }

    /// Inserts a new document into index
    /// 
    /// index: Index Name (ex: airplanes)
    /// 
    /// Data: Data to insert, data type is serde_json::Value
    /// ```
    /// Example:
    ///     json!{
    ///         "name": "test_name",
    ///         "password": "test_password",
    ///         "email": "test_email@example.com"
    ///     }
    /// ```
    /// Returns status code wheter or not it succeed, such as 404 if index not found
    /// 
    /// TODO: Return a json of statuscode + reason of failure
    pub async fn insert_document(&self, index: &str, data: Value) -> StatusCode{
        
        let response = self.elastic
            .index(IndexParts::Index(index))
            .body(data)
            .send()
            .await
            .unwrap();
        response.status_code()
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
    ///         "error_message": "Not Found",
    ///         "status_code": 404
    ///     }
    /// 
    /// Success:
    ///     json!{
    ///         "message": "Success",
    ///         "status_code": 200
    ///     }
    /// 
    /// ```
    pub async fn update_document(&self, index: Option<&str>, document_id: Option<&str>, data: Option<Value>) -> Value{

        let idx = match index{
            Some(val) => val,
            None => return json!({
                "error_message": "Index not supplied",
                "status_code": StatusCode::BAD_REQUEST.as_u16()
            })
        };

        let id = match document_id{
            Some(val) => val,
            None => return json!({
                "error_message": "Document id not supplied",
                "status_code": StatusCode::BAD_REQUEST.as_u16()
            })
        };

        let to_update = match data{
            Some(val) => val,
            None => return json!({
                "error_message": "Data not supplied",
                "status_code": StatusCode::BAD_REQUEST.as_u16()
            })
        };

        let resp = self.elastic
            .update(UpdateParts::IndexId(idx, id))
            .body(to_update)
            .send()
            .await
            .unwrap();

        println!("{:#?}", resp);
        let status_code = resp.status_code();

        let json_resp = resp.json::<Value>().await.unwrap();

        println!("{:#?}", json_resp);

        if !status_code.is_success() {
            return json!({
                "status_code": status_code.as_u16(),
                "error_message": json_resp["error"]["reason"]
            });
        }

        json!({
            "status_code": status_code.as_u16(),
            "message": json_resp["result"]
        })
    }

    /// Deletes document on an index
    /// 
    /// index: Index name (ex: airplanes)
    /// 
    /// id: Document id (ex: BnqR-YUBLRm6224CUtK7)
    /// 
    /// Returns status code whether or not it succeed, such as 200 if succeed, 404 if either index or document is not found
    pub async fn delete_document(&self, index: Option<&str>, document_id: Option<&str>) -> Value{

        let idx = match index{
            Some(val) => val,
            None => return json!({
                "error_message": "Index not supplied",
                "status_code": StatusCode::BAD_REQUEST.as_u16()
            })
        };

        let id = match document_id{
            Some(val) => val,
            None => return json!({
                "error_message": "Document id not supplied",
                "status_code": StatusCode::BAD_REQUEST.as_u16()
            })
        };

        let resp = 
            self.elastic
            .delete(DeleteParts::IndexId(idx, id))
            .send()
            .await
            .unwrap();
    
        let status_code = resp.status_code();

        let json_resp =  resp
            .json::<Value>()
            .await
            .unwrap();

        if !status_code.is_success(){
            return json!({
                "status_code": status_code.as_u16(),
                "error_message": json_resp["result"]
            });
        }

        json!({
            "status_code": status_code.as_u16(),
            "message": json_resp["result"]
        })
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
    pub async fn find_document(&self, index: &str, search_term: Option<&str>, search_on: Option<&str>, retrieve_field: Option<&str>, from: i64, count: i64) -> Value{

        let fields_to_search: Option<Vec<String>> = search_on.map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());

        let fields_to_return = match retrieve_field {
            Some(val) => val.split(',').into_iter().map(|x| x.trim().to_string()).collect(),
            None => vec!["*".to_string()],
        };

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
            }
        };

        let resp = self.elastic
            .search(SearchParts::Index(&[index]))
            .from(from)
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
            return json!({
                "status_code": status_code.as_u16(),
                "error_message": json_resp["error"]["reason"]
            });
        }
        
        json!({
            "status": status_code.as_u16(),
            "took": json_resp["took"],
            "data": json_resp["hits"]["hits"],
            "total_data": json_resp["hits"]["total"]["value"],
            "match_type": json_resp["hits"]["total"]["relation"]
        })
    }


    /// Returns a list of all index
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
        response.unwrap()
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