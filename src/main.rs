use actix_web::{web::Data, App, HttpServer};
use env_logger;
mod models;
use crate::models::client::EClient;
mod routes;
use crate::routes::*;


// TODO: Move all routes to routes folder
// TODO: Finish get

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
            // .service(get_index)
            .service(hardcoded_data_for_testing)
            .service(get_all_index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}