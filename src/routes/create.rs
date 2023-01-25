use crate::EClient;
use actix_web::{post, web::{self, Data}, Responder, Result, get};
use serde_json::Value;

/*
JSON Data Format For Create(Might change):
    {
        index: index_name
        data: {
            name
            password
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/

// Temporary hardcode to add test data

#[get("/api/hardcoded_data_add")]
pub async fn hardcoded_data_for_testing(elasticsearch_client: Data::<EClient>) -> impl Responder{

    const INDEX: &str = "airplanes_v3";
    
        
    let index_exists = elasticsearch_client.create_index(INDEX).await;

    println!("{:#?}", index_exists);

    // No question mark for await, https://github.com/actix/actix-web/wiki/FAQ
    let resp = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/algolia/datasets/master/airports/airports.json")
        .send()
        .await;

    println!("{:#?}", resp);
    println!("Test");

    let x = resp.unwrap();

    let y = x.json::<Vec<Value>>().await.unwrap();

    for k in y {
        let code = elasticsearch_client.insert_document(INDEX, k).await;
        println!("{:#?}", code);
    }
    // let successful = response.status_code().is_success();
    "Hello {app_name}!" // temp: Avoid error
}

#[post("/api/add_data")]
pub async fn add_data_to_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {
    // let (name, password) = data.into_inner();
    
    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(data.clone()))
}