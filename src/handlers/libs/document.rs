// TODO: Create bulk function
// pub async fn convert_to_bulk(operation_type: BulkTypes, data: &Value) -> {

// }

use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{handlers::errors::ErrorTypes, actions::EClientTesting};

// TODO: Convert into a lib function that uses actions
pub async fn get_document(index: &str, document_id: &str, retrieve_fields: Option<String>, client: &EClientTesting) -> Result<(StatusCode, Value), (StatusCode, ErrorTypes)>{
    let resp = client.get_document(index, document_id, retrieve_fields).await.unwrap();

    let status_code = resp.status_code();
    
    if !status_code.is_success() {
        let error = match status_code{
            StatusCode::NOT_FOUND => ErrorTypes::DocumentNotFound(document_id.to_string()),
            _ => ErrorTypes::Unknown
        };
        return Err((status_code, error));
    }

    let json_resp = resp.json::<Value>().await.unwrap();

    Ok((status_code, json_resp))
}

pub fn search_body_builder(search_term: Option<String>, search_in: Option<String>, retrieve_field: Option<String>, include_source: bool, fuzziness: Option<String>) -> Value{
    let fields_to_search: Option<Vec<String>> = search_in.map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());

    let fields_to_return = match retrieve_field {
        Some(val) => val.split(',').into_iter().map(|x| x.trim().to_string()).collect(),
        None => vec!["*".to_string()],
    };

    // Returns everything
    let mut body = json!({
        "_source": include_source,
        "query": {
            "match_all": {} 
        },
        "fields": fields_to_return
    });

    // Some(temp_variable) = existing_variable {function(temp_variable from existing_variable)}
    if let Some(search) = search_term {
        if let Some(search_field) = fields_to_search {
            body = json!({
                "_source": include_source,
                "query": {
                    "multi_match": {
                        "query": search,
                        "fields": search_field,
                        "fuzziness": fuzziness.unwrap_or("AUTO".to_string())     
                    }
                },
                "fields": fields_to_return
            })
        } else {
            body = json!({
                "_source": include_source,
                "query": {
                    "multi_match": {
                        "query": search,
                        "fields": "*",
                        "fuzziness": fuzziness.unwrap_or("AUTO".to_string())     
                    }
                },
                "fields": fields_to_return
            });
        }
    };

    return body;
}
