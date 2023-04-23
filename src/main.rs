// use std::task::Poll;
// use std::time::Duration;

// use std::sync::Arc;

use actions::EClient;
// use actix_web::rt::time::sleep;
// use actix_web::{HttpResponse};
use actix_web::{web::{self, Data}, App, HttpServer};
// use futures::FutureExt;
// use futures::future::join_all;
use handlers::{application::{initialize_new_app_id, get_application_list, get_application, delete_application, update_application,}, for_testing::{create_test_data::test_data, file_import::create_by_file, destructive_delete_all::delete_everything}, welcome::welcome, index::{get_app_list_of_indexes, get_index}};
use handlers::document::{post_search, search, update_document, delete_document, get_document, create_bulk_documents};
// use handlers::for_testing::get_keys::test_get_keys;
use handlers::index::{create_index, update_mappings, get_mappings, delete_index};
// use handlers::libs::is_server_up;
// use serde::Deserialize;
// use serde_json::{Value, json};
mod middlewares;
use middlewares::cors::cors;

mod actions;
mod handlers;

// TODO: Read config from file
pub const APPLICATION_LIST_NAME: &str = "application_list";
pub const DEFAULT_ELASTIC_SHARDS: usize = 3;
pub const DEFAULT_ELASTIC_REPLICAS: usize = 3;
pub const DEFAULT_PARTITIONS: usize = 10;

/// 50MB in bytes
pub const MAX_FILE_SIZE: usize = 1024 * 1024 * 50;

// ? TODO: A Loop that checks the apps list every 5 or so seconds and stores it in a struct which is accessible by every function
// ! Potential Problem: https://discuss.elastic.co/t/how-to-handle-lots-of-small-indices/272063
// ! -> Too many indices can crash elasticsearch

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    // std::env::set_var("RUST_LOG", "debug");
    env_logger::init();


    let client = Data::new(EClient::new("http://127.0.0.1:9200"));

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
                .route("/index/list/{app_id}", web::get().to(get_app_list_of_indexes))
                .route("/index/mappings/{app_id}/{index}", web::get().to(get_mappings))
                .route("/index/mappings", web::put().to(update_mappings))
                .route("/index/{app_id}/{index}", web::delete().to(delete_index))
                
                .route("/document/{app_id}/{index}", web::post().to(create_bulk_documents))
                // .route("/document/bulk/{app_id}/{index}", web::post().to(create_bulk_documents))
                .route("/document/{app_id}/{index}/{document_id}", web::get().to(get_document))
                .route("/search/{app_id}/{index}", web::post().to(post_search))
                .route("/search/{app_id}/{index}", web::get().to(search))
                .route("/document/{app_id}/{index}/{document_id}", web::put().to(update_document))
                .route("/document/{app_id}/{index}/{document_id}", web::delete().to(delete_document))
        
                .service(
                    web::scope("/another_test")
                        .route("/test_data/{app_id}", web::post().to(test_data))
                        .route("/file_test/{app_id}/{index}", web::get().to(create_by_file))
                        .route("/delete/destructive_delete_all", web::delete().to(delete_everything))
                        // .route("/get_index/{app_id}/{index}", web::get().to(test_get_index))
                        // .route("/document/{app_id}/{index}", web::put().to(update_bulk_documents))
                )
                .route("", web::get().to(welcome))
                .route("/", web::get().to(welcome))
        )
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


}