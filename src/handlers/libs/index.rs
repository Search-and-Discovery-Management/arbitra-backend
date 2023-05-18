use futures::{future::join_all};
use reqwest::StatusCode;
use serde_json::{json};

use crate::{actions::EClient, handlers::errors::ErrorTypes, AppConfig};

use super::application::get_app_indexes_list;


/// Inserts a new index into an application
pub async fn create_or_exists_index(app_id: Option<String>, index: &str, shards: Option<usize>, replicas: Option<usize>, partition: Option<usize>, client: &EClient, app_config: &AppConfig) -> StatusCode {

    // Check if index exists
    let index_name = match &app_id {
        Some(x) => index_name_builder(x, index).to_string(),
        None => index.to_string()
    };

    let exists = client.check_index(&index_name).await.unwrap();

    // If exists, return conflict
    if exists.status_code().is_success() {
        return StatusCode::CONFLICT;
    }

    // If not found, create a new index
    if exists.status_code() == StatusCode::NOT_FOUND {
        let body = 
            json!(
                {
                    "mappings": { 	
                        "dynamic":"true"
                    },
                    "settings": {
                        "index.number_of_shards": shards.unwrap_or(app_config.default_elastic_shards),
                        "index.number_of_replicas": replicas.unwrap_or(app_config.default_elastic_replicas),
                    }
                }
            );

            let mut statuses: Vec<StatusCode> = vec![];
            
            if app_id.is_none(){
                statuses.push(client.create_index(index_name, &body).await.unwrap().status_code());
            } else {
                let mut indexes: Vec<_> = vec![];

                // Creates num of partitions in a row
                for i in 0..partition.unwrap_or(app_config.default_partitions) {
                    indexes.push(client.create_index(format!("{index_name}.{i}"), &body))
                }

                let resp = join_all(indexes).await;

                statuses = resp.iter().map(|x| x.as_ref().unwrap().status_code()).collect();
            }

        if !statuses.iter().any(|x| !x.is_success()){
            return StatusCode::CREATED;
        }
    }

    exists.status_code()
}

/// Formats the name into app_id.index_name
pub fn index_name_builder(app_id: &str, index_name: &str) -> String{
    format!("{}.{}", app_id.trim().to_ascii_lowercase(), index_name.trim().to_ascii_lowercase())
}

pub async fn index_exists(app_id: &str, index_name: &str, client: &EClient, app_config: &AppConfig) -> Result<(usize, Vec<String>), (StatusCode, ErrorTypes, Vec<String>)> {
    let list = match get_app_indexes_list(app_id, client, app_config).await {
        Ok(x) => x,
        Err((status, error)) => return Err((status, error, Vec::new()))
    };

    match list.iter().position(|x| x.eq(index_name)) {
        Some(x) => Ok((x, list)),
        None => Err((StatusCode::NOT_FOUND, ErrorTypes::IndexNotFound(index_name.to_string()), list))
    }
}