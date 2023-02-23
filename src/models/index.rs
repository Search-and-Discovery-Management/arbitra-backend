use actix_web::HttpResponse;
use elasticsearch::{indices::{IndicesExistsParts, IndicesCreateParts, IndicesPutMappingParts, IndicesGetMappingParts, IndicesDeleteParts}, cat::CatIndicesParts};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::models::ErrorTypes;

use super::{EClient, helpers::{server_down_check, index_exists_check}};



impl EClient{
    /// Creates a new index
    /// 
    /// index: Index Name
    pub async fn create_index(&self, index: &str) -> HttpResponse{

        match server_down_check(&self.elastic).await{
            Ok(()) => (),
            Err(x) => return x
        };

        // Check if index exists
        let exists = self.elastic
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await
            .unwrap();

        if exists.status_code().is_success() {
            return HttpResponse::Conflict().json(json!({"error": ErrorTypes::IndexExists(index.to_string()).to_string()}));
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

    pub async fn update_index_mappings(&self, index: &str, mappings: Value) -> HttpResponse{

        match server_down_check(&self.elastic).await{
            Ok(()) => (),
            Err(x) => return x
        };

        match index_exists_check(&self.elastic, index).await{
            Ok(()) => (),
            Err(x) => return x
        };

        let resp = self.elastic
            .indices()
            .put_mapping(IndicesPutMappingParts::Index(&[index]))
            .body(mappings)
            .send()
            .await
            .unwrap();

        let status_code = resp.status_code();

        if !status_code.is_success() {
            let error = match status_code{
                StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
                _ => ErrorTypes::Unknown.to_string()
            };
            return HttpResponse::build(status_code).json(json!({
                "error": error
            }));
        }

        // println!("{:#?}", resp);
        // println!("{:#?}", resp.json::<Value>().await.unwrap());

        HttpResponse::build(status_code).finish()
    }

    /// Returns either a list of index if index is not supplied, or the specified index
    /// index: index name
    pub async fn get_index(&self, index: Option<String>) -> HttpResponse{

        match server_down_check(&self.elastic).await{
            Ok(()) => (),
            Err(x) => return x
        };

        if index.is_some() {
            match index_exists_check(&self.elastic, &index.clone().unwrap()).await{
                Ok(()) => (),
                Err(x) => return x
            };
        }

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

        if !status_code.is_success() {
            return HttpResponse::build(status_code).json(json!({
                "error": ErrorTypes::Unknown.to_string()
            }));
        }
        
        let json_resp = resp.json::<Value>().await.unwrap();

        HttpResponse::build(status_code).json(json_resp)
    }

    /// Returns the mappings of an index
    pub async fn get_index_mappings(&self, index: String) -> HttpResponse{
        match server_down_check(&self.elastic).await{
            Ok(()) => (),
            Err(x) => return x
        };

        match index_exists_check(&self.elastic, &index).await{
            Ok(()) => (),
            Err(x) => return x
        };

        let resp = self.elastic
            .indices()
            .get_mapping(IndicesGetMappingParts::Index(&[&index]))
            .send()
            .await
            .unwrap();

        let status_code = resp.status_code();

        if !status_code.is_success() {
            return HttpResponse::build(status_code).json(json!({
                "error": ErrorTypes::Unknown.to_string()
            }));
        }

        let json_resp = 
            resp.json::<Value>()
            .await
            .unwrap();


        let mappings = json_resp
            .get(&index)
            .unwrap()
            .get("mappings")
            .unwrap();

        HttpResponse::build(status_code).json(mappings)
    }
    
    /// Deletes an index
    pub async fn delete_index(&self, index: &str) -> HttpResponse{
        match server_down_check(&self.elastic).await{
            Ok(()) => (),
            Err(x) => return x
        };

        match index_exists_check(&self.elastic, index).await{
            Ok(()) => (),
            Err(x) => return x
        };

        let resp = 
            self.elastic
            .indices()
            .delete(IndicesDeleteParts::Index(&[index]))
            .send()
            .await;

        let resp = resp.unwrap();
    
        let status_code = resp.status_code();

        if !status_code.is_success(){
            let json_resp =  resp
                .json::<Value>()
                .await
                .unwrap();

            return HttpResponse::build(status_code).json(
                    json!({
                        "message": json_resp["error"]["reason"]
                    })
                );
        }

        HttpResponse::build(status_code).finish()
    }

}