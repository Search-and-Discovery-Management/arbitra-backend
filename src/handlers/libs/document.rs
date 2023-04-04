// ? TODO: Create bulk function
// pub async fn convert_to_bulk(operation_type: BulkTypes, data: &Value) -> {

use actix_web::HttpResponse;
// }
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{handlers::{errors::ErrorTypes, libs::index_name_builder}, actions::EClientTesting};

pub async fn get_document(index: &str, document_id: &str, retrieve_fields: &Option<String>, client: &EClientTesting) -> Result<(StatusCode, Value), (StatusCode, ErrorTypes)>{
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

pub async fn document_search(app_id: &str, index: &str, body: &Value, from: &Option<i64>, count: &Option<i64>, client: &EClientTesting) -> Result<(StatusCode, Value), HttpResponse> {

    let name = index_name_builder(app_id, index);

    let resp = client.search_index(&name, &body, &from, &count).await.unwrap();

    let status = resp.status_code();

    if !status.is_success() {
        let error = match status {
            StatusCode::NOT_FOUND => ErrorTypes::IndexNotFound(index.to_owned()).to_string(),
            StatusCode::BAD_REQUEST => ErrorTypes::BadDataRequest.to_string(),
            _ => ErrorTypes::Unknown.to_string()
        };

        return Err(HttpResponse::build(status).json(json!({"error": error})));
    };

    // let json_resp = resp.json::<Value>().await.unwrap();
    let convert_time = std::time::Instant::now();
    let json_resp = resp.json::<Value>().await.unwrap();
    println!("{:#?}", convert_time.elapsed().as_millis());
    
    return Ok((status, json_resp))
}


pub fn search_body_builder(search_term: &Option<String>, search_in: &Option<Vec<String>>, retrieve_field: &Option<String>) -> Value{ // , fuzziness: Option<String>) -> Value{
    // let fields_to_search: Option<Vec<String>> = search_in.map(|val| val.split(',').into_iter().map(|x| x.trim().to_string()).collect());

    // let fields_to_search = search_in.as_deref().unwrap_or(&vec!["*".to_string()]);
    let fields_to_search = search_in.to_owned().unwrap_or(vec!["*".to_string()]);

    let fields_to_return = match retrieve_field {
        Some(val) => val.split(',').into_iter().map(|x| x.trim().to_string()).collect(),
        None => vec!["*".to_string()],
    };

    // Returns everything
    let mut body = json!({
        "_source": {
            "includes": fields_to_return
        },
        "query": {
            "match_all": {} 
        },
    });
    // "fields": fields_to_return
    // "fields": fields_to_return

    // Some(temp_variable) = existing_variable {function(temp_variable from existing_variable)}
    // if let Some(search_field) = fields_to_search {
    //     body = json!({
    //         "_source": include_source,
    //         "query": {
    //             "multi_match": {
    //                 "query": search,
    //                 "fields": fields_to_search,
    //                 "fuzziness": fuzziness.unwrap_or("AUTO".to_string())     
    //             }
    //         },
    //         "fields": fields_to_return
    //     })
    // } else {
        if let Some(term) = search_term {
            body = json!({
                "_source": {
                    "includes": fields_to_return
                },
                "query": {
                        "query_string": {
                            "query": term,
                            "type": "cross_fields",
                            "fields": fields_to_search,
                            "minimum_should_match": "75%"
                        }
                    }
                })
            // "fields": fields_to_return
            // "operator": "or"
            // "operator": "and"    
        }

        // Testing copy_to 
        // if let Some(term) = search_term {
        //     if let Some(fields_to_search) = search_in {
        //         // "_source": true,
        //         body = json!({
        //         "_source": {
        //             "includes": fields_to_return
        //         },
        //         "query": {
        //                 "multi_match": {
        //                     "query": term,
        //                     "fields": fields_to_search,
        //                     "fuzziness": fuzziness.unwrap_or("AUTO".to_string()),
        //                 }
        //             },
        //             "fields": fields_to_return
        //         })
        //         // "operator": "and"    
        //     } else {
        //         // "_source": true,
        //         // "operator": "and"
        //         println!("{:#?}", fields_to_return);
        //         println!("{:#?}", search_in);
        //         body = json!({
        //             "_source": {
        //                 "includes": fields_to_return
        //             },
        //             "query": {
        //                 "match": {
        //                     "default_search": {
        //                         "query": term,
        //                         "operator": "or"
        //                     }
        //                 }
        //             }
        //         })
        //     };
        // };
        // "fuzziness": fuzziness.unwrap_or("AUTO".to_string()),
        // "fields": fields_to_return
        // }
        // "fields": fields_to_return
        
    body
}
