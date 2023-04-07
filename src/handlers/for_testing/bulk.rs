use actix_web::{HttpResponse, web::{self, Data}};
use reqwest::StatusCode;
// use serde::Deserialize;
use serde_json::{Value, json};

use crate::{actions::EClientTesting, handlers::{libs::{check_server_up_exists_app_index, index_name_builder}, index_struct::RequiredIndex}};

// #[derive(Deserialize)]
// pub struct CreateBulkDocuments{
//     pub app_id: String,
//     pub index: String,
//     pub data: Vec<Value>,
//     // pub dynamic_mode: Option<String>
// }

/// TODO: Turn into an actually usable function
/// 
/// ? Return the errors? or only the count of errors?
/// 
/// TODO: Loop and check what type of errors are being thrown, if there is only one, turn that into the status error, else use MULTI STATUS
/// 
/// Bulk Document Create Input, Only allows input into an existing index
pub async fn testing_create_bulk_documents(app_index: web::Path<RequiredIndex>, data: web::Json<Vec<Value>>, client: Data::<EClientTesting>) -> HttpResponse {

    // create_or_exists_index(Some(data.app_id.to_string()), &data.index, None, None, &client).await;

    let idx = app_index.index.clone().trim().to_ascii_lowercase();

    // let test = json!(vec!["test"]);


    match check_server_up_exists_app_index(&app_index.app_id, &idx, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let name = index_name_builder(&app_index.app_id, &idx);

    let resp = client.bulk_create_documents(&name, &data).await.unwrap();

    let status = resp.status_code();
    let json: Value = resp.json::<Value>().await.unwrap();

    if json["errors"].as_bool().unwrap() {
        let failed: Vec<&Value> = json["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| !v["error"].is_null())
            .collect();

        println!("Errors whilst indexing. Failures: {}", failed.len());
        return HttpResponse::build(StatusCode::MULTI_STATUS).json(serde_json::json!({
            "error_count": failed.len(),
            "errors": failed
        })
        )
    }
    
    HttpResponse::build(status).finish()
}