use reqwest::{Url, StatusCode};
use serde_json::{Value, json};
use actix_web::{get, post, web::{self, Data}, App, HttpServer, Responder, Result, delete, put};
use serde::{Serialize, Deserialize};
use elasticsearch::{
    Elasticsearch,
    Error,
    http::transport::{TransportBuilder,SingleNodeConnectionPool}, 
    indices::{IndicesExistsParts, IndicesCreateParts},
};

// #[derive(Serialize, Debug)]
// struct settings {
//     number_of_shards: u32,
//     number_of_replicas: u32
// }

#[derive(Serialize, Clone, Deserialize)]
struct SearchTermJson{
    search_term: String,
    count: u64
}


#[derive(Serialize, Clone, Deserialize)]
struct NewData{
    // TODO: Datatypes
    id: u64,
}

#[post("/api/add_data")]
async fn add_data_to_index(data: web::Json<NewData>, client: Data<Elasticsearch>) -> Result<impl Responder> {
    // let (name, password) = data.into_inner();

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(data.clone()))
}

#[put("/api/update_data")]
async fn update_data_on_index(updated_data: web::Json<NewData>, client: Data<Elasticsearch>) -> Result<impl Responder> {
    // Update data in index

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(updated_data.clone()))
}

#[delete("/api/delete_data")]
async fn delete_data_on_index(search_term: web::Json<SearchTermJson>, client: Data<Elasticsearch>) -> Result<impl Responder> {

    // Deletes the data inside the index

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(search_term.clone()))
}

#[get("/api/get_index")]
async fn get_index(search_term: web::Json<SearchTermJson>, client: Data<Elasticsearch>) -> Result<impl Responder> {

    // if exists: return values in index

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(search_term.clone()))
}

// #[get("/api/get_all_available_index")]
// async fn get_all_available_index(search_term: web::Data<SearchTermJson>, client: Data<Elasticsearch>) -> Result<impl Responder> {

//     // if exists: return all available indexes
//     let index_to_find = data;

//     // Ok(web::Json(users.user_list.clone()))

// }

#[post("/api/index_search")]
async fn search_in_index(data: web::Json<SearchTermJson>, client: Data<Elasticsearch>) -> Result<impl Responder> {
    // let (name, password) = data.into_inner();
    let index_to_find = data;

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(index_to_find.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = create_client().unwrap();
    // let mut x = Data::new(Users{
    //     user_list: vec![
    //         User{
    //             name:"Test".to_string(), 
    //             password: "Password".to_string()
    //     }]
    // });
    HttpServer::new(move || {
        App::new()
            // .app_data(x.clone())
            .app_data(client.clone())
            .service(add_data_to_index)
            .service(update_data_on_index)
            .service(delete_data_on_index)
            // .service(get_all_available_index)
            .service(search_in_index)
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
    let exists = client
        .indices()
        .exists(IndicesExistsParts::Index(&[index]))
        .send()
        .await?;

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