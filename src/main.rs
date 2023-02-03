use actix_web::{web::Data, App, HttpServer};
mod models;
use crate::models::client::EClient;
mod routes;
use crate::routes::*;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(EClient::new("http://127.0.0.1:9200")))
            .service(add_data_to_index) // #[post("/api/create_document")]
            .service(update_data_on_index) // #[put("/api/update_document")]
            .service(delete_data_on_index) // #[delete("/api/delete_document")]
            .service(search_in_index) // #[get("/api/search_documents")]
            // .service(get_index)
            .service(hardcoded_data_for_testing) // #[get("/api/hardcoded_data_add")]
            .service(create_new_index) // #[post("/api/create_index")]
            .service(get_all_index) // #[get("/api/get_index_list")]
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}