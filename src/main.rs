use serde_json::{Value};
use actix_web::{get, post, web::{self, Data}, App, HttpServer, Responder, Result, delete, put};
use env_logger;
mod models;
use crate::models::client::EClient;

// TODO: Move all routes to routes folder
// TODO: Finish get

/*
JSON Data Format For Create(Might change):
    {
        Index: index_name
        Data: {
            name
            password
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/

/*
JSON Data Format For Update(Might change):
    {
        Index: index_name
        Data: {
            id
            name
            password
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/

/*

JSON Data Format For Delete(Might change):
    {
        Index: index_name
        Document_id: id
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/

/*

JSON Data Format For Get(Might change):
    {
        Index: index_name
        SearchTerm: ABC
        Search_in: (field_name)
        Return_fields: {
            id
            name
            password
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/

// Temporary hardcode to add test data, Not Finished



#[get("/api/hardcoded_data_add")]
async fn hardcoded_data_for_testing(elasticsearch_client: Data::<EClient>) -> impl Responder{
    // let (name, password) = data.into_inner();
    
    // elasticsearch_client.

    const INDEX: &str = "airplanes_v3";

    // let body: Vec<BulkOperation<_>> = 
        // let body: Vec<BulkOperation<_>> = vec![];
    
        
    let index_exists = elasticsearch_client.create_index(INDEX).await;
    //(&elasticsearch_client, INDEX).await;

    println!("{:#?}", index_exists);


    // No question mark for await, https://github.com/actix/actix-web/wiki/FAQ
    let resp = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/algolia/datasets/master/airports/airports.json")
        .send()
        .await;
        // .unwrap()
        // .json::<std::collections::HashMap<String, String>>()
        // .await
        // .unwrap();
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
async fn add_data_to_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {
    // let (name, password) = data.into_inner();
    
    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(data.clone()))
}

#[put("/api/update_data")]
async fn update_data_on_index(updated_data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {
    // Update data in index

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(updated_data.clone()))
}

#[delete("/api/delete_data")]
async fn delete_data_on_index(search_term: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    // Deletes the data inside the index

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(search_term.clone()))
}

#[get("/api/get_index")]
async fn get_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    // if exists: return values in index

    println!("{:#?}", data);
    let index = data.get("index");
    if index == None {
        println!("{:#?}", index);
        println!("Fail");

    }
    
    // let resp = elasticsearch_client.find_document(index, search_term, fields).await;

    // println!("{:#?}", resp);
    Ok(web::Json(data.clone()))

}

#[post("/api/find_in_index")]
async fn search_in_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    // // let index_to_find = data;
    // // println!("{:#?}", index_to_find);
    // // Ok(web::Json(index_to_find.clone()))

    // println!("{:#?}", data);
    let index = data.get("index");
    if index == None {
        println!("{:#?}", index);
        println!("Fail");

    }

    let search_term = data.get("SearchTerm");
    if search_term == None {
        println!("{:#?}", search_term);
        println!("Search term fail");
    }

    // let resp = elasticsearch_client
    // .search(SearchParts::Index(&[&(index.unwrap().to_string())]))
    // .body(json!({
    //     "query": {
    //         "match": {
    //             "body": &search_term
    //         }
    //     }
    // }))
    // .send()
    // .await; // missing "?"

    Ok(web::Json(data.clone()))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(EClient::new("http://127.0.0.1:9201")))
            .service(add_data_to_index)
            .service(update_data_on_index)
            .service(delete_data_on_index)
            .service(search_in_index)
            .service(get_index)
            .service(hardcoded_data_for_testing)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}