// use std::task::Poll;
// use std::time::Duration;

// use std::sync::Arc;

use actions::EClientTesting;
// use actix_web::rt::time::sleep;
// use actix_web::{HttpResponse};
use actix_web::{web::{self, Data}, App, HttpServer};
// use futures::FutureExt;
// use futures::future::join_all;
use handlers::{application::{initialize_new_app_id, get_application_list, get_application, delete_application, update_application}, for_testing::{create_test_data::test_data, bulk::testing_create_bulk_documents}};
use handlers::document::{create_document, post_search, search, update_document, delete_document, get_document};
// use handlers::for_testing::get_keys::test_get_keys;
use handlers::index::{get_index, create_index, update_mappings, get_mappings, delete_index};
// use handlers::libs::is_server_up;
// use serde::Deserialize;
// use serde_json::{Value, json};
mod middlewares;
use middlewares::cors::cors;

mod actions;
mod handlers;

pub const APPLICATION_LIST_NAME: &str = "application_list";

// ? TODO: A Loop that checks the apps list every 5 or so seconds and stores it in a struct which is accessible by every function
// ? TODO: Convert all functions into MLI-Compatible functions
// ! Potential Problem: https://discuss.elastic.co/t/how-to-handle-lots-of-small-indices/272063
// ! -> Too many indices can crash elasticsearch

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();


    let client = Data::new(EClientTesting::new("http://127.0.0.1:9200"));

    // Start server
    HttpServer::new(move || {
        App::new()
        .wrap(cors())
        .service(
            web::scope("/api")
                .app_data(client.clone())
                .route("/app", web::post().to(initialize_new_app_id))
                .route("/apps", web::get().to(get_application_list))
                .route("/app/{app_id}", web::get().to(get_application))
                .route("/app", web::put().to(update_application))   
                .route("/app/{app_id}", web::delete().to(delete_application))
                
                .route("/index/{app_id}", web::post().to(create_index))
                .route("/index/{app_id}", web::get().to(get_index))
                .route("/index/mappings/{app_id}/{index}", web::get().to(get_mappings))
                .route("/index/mappings", web::put().to(update_mappings))
                .route("/index/{app_id}/{index}", web::delete().to(delete_index))
        
                .route("/document/{app_id}/{index}", web::post().to(create_document))
                .route("/document/{app_id}/{index}/{document_id}", web::get().to(get_document))
                .route("/search", web::post().to(post_search))
                .route("/search/{app_id}/{index}", web::get().to(search))
                .route("/document", web::put().to(update_document))
                .route("/document/{app_id}/{index}/{document_id}", web::delete().to(delete_document))
        
                .service(
                    web::scope("/another_test")
                    // .route("/get_keys", web::get().to(test_get_keys))
                    .route("/test_data/{app_id}", web::post().to(test_data))
                    .route("/bulk_add_data/{app_id}/{index}", web::post().to(testing_create_bulk_documents))
                )
        )
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


}