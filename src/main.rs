use reqwest::{Url, StatusCode};
use serde_json::{Value, json};
use actix_web::{get, post, web::{self, Data}, App, HttpServer, Responder, Result, delete, put};
// use serde::{Serialize, Deserialize};
use elasticsearch::{
    Elasticsearch,
    Error,
    http::transport::{TransportBuilder,SingleNodeConnectionPool}, 
    indices::{IndicesExistsParts, IndicesCreateParts}, SearchParts,
};
use env_logger;

// #[derive(Serialize, Clone, Deserialize)]
// struct SearchTermJson{
//     search_term: String,
//     count: u64
// }

// #[derive(Serialize, Clone, Deserialize)]
// struct NewData{
//     // TODO: Datatypes
//     id: u64,
// }

/*

JSON Data Format (Might change):
    {
        Index: index_name
        Operation: PUT
        Data: {
            id
            name
            password
            etc
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/


/*

JSON Data Format (Might change):
    {
        Index: index_name
        Operation: GET
        SearchTerm: ABC
        Data: {
            id
            name
            password
            etc
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/


#[post("/api/add_data")]
async fn add_data_to_index(data: web::Json<Value>, elasticsearch_client: Data::<Elasticsearch>) -> Result<impl Responder> {
    // let (name, password) = data.into_inner();
    
    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(data.clone()))
}

#[put("/api/update_data")]
async fn update_data_on_index(updated_data: web::Json<Value>, elasticsearch_client: Data::<Elasticsearch>) -> Result<impl Responder> {
    // Update data in index

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(updated_data.clone()))
}

#[delete("/api/delete_data")]
async fn delete_data_on_index(search_term: web::Json<Value>, elasticsearch_client: Data::<Elasticsearch>) -> Result<impl Responder> {

    // Deletes the data inside the index

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(search_term.clone()))
}

#[get("/api/get_index")]
async fn get_index(data: web::Json<Value>, elasticsearch_client: Data::<Elasticsearch>) -> Result<impl Responder> {

    // if exists: return values in index
    // println!("{:#?}", search_term.clone().as_array());
    // println!("{:#?}", search_term.clone());


    // let root = search_term.get("settings");
    
    println!("{:#?}", data);
    let index = data.get("index");
    if index == None {
        println!("{:#?}", index);
        println!("Fail");

    }

    let resp = elasticsearch_client
    .search(SearchParts::Index(&[&(index.unwrap().to_string())]))
    .body(json!({
        "query": {
            "match": {
                // "body": &search_term
            }
        }
    }))
    .send()
    .await; // missing "?"
    
    // let resp = elasticsearch_client
    //     .search(SearchParts::Index(&[search_term.get("index")]))
    //     .body(json!({
    //         "query": {
    //             "match": {
    //                 "body": search_term.get("SearchTerm")
    //             }
    //         }
    //     }));


    println!("{:#?}", resp);
    Ok(web::Json(data.clone()))

}

#[post("/api/find_in_index")]
async fn search_in_index(data: web::Json<Value>, elasticsearch_client: Data::<Elasticsearch>) -> Result<impl Responder> {

    // let index_to_find = data;
    // println!("{:#?}", index_to_find);
    // Ok(web::Json(index_to_find.clone()))

    println!("{:#?}", data);
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

    let resp = elasticsearch_client
    .search(SearchParts::Index(&[&(index.unwrap().to_string())]))
    .body(json!({
        "query": {
            "match": {
                "body": &search_term
            }
        }
    }))
    .send()
    .await; // missing "?"

    Ok(web::Json(data.clone()))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Create ElasticSearch client
    let client = create_client().unwrap();

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(add_data_to_index)
            .service(update_data_on_index)
            .service(delete_data_on_index)
            .service(search_in_index)
            .service(get_index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn create_client() -> Result<Elasticsearch, Error> {
    let url = Url::parse("http://127.0.0.1:9201").unwrap();

    let conn_pool = SingleNodeConnectionPool::new(url);
    let builder = TransportBuilder::new(conn_pool);

    let transport = builder.build()?;
    Ok(Elasticsearch::new(transport))
}

async fn create_index(client: &Elasticsearch, index: &str) -> Result<(), Error> {
    // Check if index exists
    let exists = client
        .indices()
        .exists(IndicesExistsParts::Index(&[index]))
        .send()
        .await?;

    // If doesnt exist, create
    if exists.status_code() == StatusCode::NOT_FOUND {
        let response = client
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