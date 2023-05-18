use actions::EClient;
use actix_web::{web::{self, Data}, App, HttpServer};
use handlers::{application::{initialize_new_app_id, get_application_list, get_application, delete_application, update_application,}, for_testing::{create_test_data::test_data, destructive_delete_all::delete_everything, bulk::{update_bulk_documents, test_update}, index_get_all::get_indexes_list_debug}, welcome::welcome, index::{get_app_list_of_indexes, get_index}, document::create_by_file};
use handlers::document::{post_search, search, update_document, delete_document, get_document, create_bulk_documents};
use handlers::index::{create_index, update_mappings, get_mappings, delete_index};
mod middlewares;
use middlewares::cors::cors;

mod actions;
mod handlers;

pub struct AppConfig {
    application_list_name: String,
    default_elastic_shards: usize,
    default_elastic_replicas: usize,
    default_partitions: usize,
    max_input_file_size: usize
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Default
    // TODO: Read config from file
    let app_config = AppConfig {
        application_list_name: "application_list".to_string(),
        default_elastic_shards: 3,
        default_elastic_replicas: 3,
        default_partitions: 10,
        max_input_file_size: 1024 * 1024 * 50
    };

    let client = Data::new(EClient::new("http://127.0.0.1:9200"));
    let data_cfg = Data::new(app_config);

    // Start server
    HttpServer::new(move || {
        App::new()
        .wrap(cors())
        .service(
            web::scope("/api")
                .app_data(client.clone())
                .app_data(data_cfg.clone())
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
                .route("/document/{app_id}/{index}/{document_id}", web::get().to(get_document))
                .route("/search/{app_id}/{index}", web::post().to(post_search))
                .route("/search/{app_id}/{index}", web::get().to(search))
                .route("/document/{app_id}/{index}/{document_id}", web::put().to(update_document))
                .route("/document/{app_id}/{index}/{document_id}", web::delete().to(delete_document))
                .route("/document/upload/{app_id}/{index}", web::get().to(create_by_file))
        
                .service(
                    web::scope("/another_test")
                        .route("/test_data/{app_id}", web::post().to(test_data))
                        .route("/delete/destructive_delete_all", web::delete().to(delete_everything))
                        .route("/update/bulk/update/{app_id}/{index}", web::put().to(test_update))
                        .route("/document/{app_id}/{index}", web::put().to(update_bulk_documents))
                        .route("/index_list", web::get().to(get_indexes_list_debug))
                )
                .route("", web::get().to(welcome))
                .route("/", web::get().to(welcome))
        )
        })
    .bind(("127.0.0.1", 7777))?
    .run()
    .await


}
