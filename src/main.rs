use actions::EClientTesting;
use actix_web::web;
use actix_web::{web::Data, App, HttpServer};
use handlers::application::{initialize_new_app_id, get_application_list, get_application, delete_application, update_application};
use handlers::index::{_get_index, _create_index, _update_mappings, _get_mappings, _delete_index};
mod models_backup;

use crate::models_backup::client::EClient;
mod routes_backup;
use crate::routes_backup::*;

mod actions;
mod handlers;

pub const APPLICATION_LIST_NAME: &str = "application_list";

// TODO: A Loop that checks the apps list every minute and stores it in a struct which is accessible by every function


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Start server
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .app_data(Data::new(EClient::new("http://127.0.0.1:9200")))
                    .route("/document/{index}/{document_id}", web::get().to(get_document))
                    .route("/document", web::post().to(create_document))
                    .route("/document", web::put().to(update_document))
                    .route("/document/{index}/{document_id}", web::delete().to(delete_document))

                    .route("/search/{index}", web::get().to(search))
                    .route("/search", web::post().to(post_search))
                    
                    .route("/index", web::get().to(get_index))
                    .route("/index", web::post().to(create_index))
                    .route("/index/{index}", web::delete().to(delete_index))

                    .route("/mappings/{index}", web::get().to(get_mapping))
                    .route("/mappings", web::put().to(update_mapping))

                    // #[delete("/api/document/{index}/{document_id}")]
                    .route("/welcome", web::get().to(welcome))

                    // Temporary
                    .service(
                        web::scope("/test")
                        .route("/add_data", web::get().to(hardcoded_data_for_testing))   
                    )
                    .service(
                        web::scope("/mli_test")
                            .app_data(Data::new(EClientTesting::new("http://127.0.0.1:9200")))
                            .route("/app", web::post().to(initialize_new_app_id))
                            .route("/apps", web::get().to(get_application_list))
                            .route("/app/{app_id}", web::get().to(get_application))
                            .route("/app", web::post().to(update_application))   
                            .route("/app/{app_id}", web::delete().to(delete_application))
                            
                            .route("/index", web::post().to(_create_index))
                            .route("/index/{app_id}", web::get().to(_get_index))
                            .route("/index/mappings", web::get().to(_get_mappings))
                            .route("/index/mappings", web::post().to(_update_mappings))
                            .route("/index/{app_id}/{index}", web::delete().to(_delete_index))
                            // .route("/index/{app_id}/{index}")
                        //     #[derive(Deserialize)]
                        // pub struct IndexDelete {
                        //     pub app_id: String,
                        //     pub index: String
                        // }
            ))
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}