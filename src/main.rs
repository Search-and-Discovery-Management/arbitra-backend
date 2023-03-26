// use std::task::Poll;
// use std::time::Duration;

// use std::sync::Arc;

use actions::EClientTesting;
// use actix_web::rt::time::sleep;
// use actix_web::{HttpResponse};
use actix_web::{web::{self, Data}, App, HttpServer};
// use futures::FutureExt;
// use futures::future::join_all;
use handlers::{application::{initialize_new_app_id, get_application_list, get_application, delete_application, update_application}, for_testing::create_test_data::test_data};
use handlers::document::{_create_document, _post_search, _search, _update_document, _delete_document, _get_document};
use handlers::for_testing::get_keys::test_get_keys;
use handlers::index::{_get_index, _create_index, _update_mappings, _get_mappings, _delete_index};
// use handlers::libs::is_server_up;
// use serde::Deserialize;
// use serde_json::{Value, json};
mod models_backup;

use crate::models_backup::client::EClient;
mod routes_backup;
use crate::routes_backup::*;

mod actions;
mod handlers;

pub const APPLICATION_LIST_NAME: &str = "application_list";

// ? TODO: A Loop that checks the apps list every 5 or so seconds and stores it in a struct which is accessible by every function
// ? TODO: Convert all functions into MLI-Compatible functions
// ! Potential Problem: https://discuss.elastic.co/t/how-to-handle-lots-of-small-indices/272063
// ! -> Too many indices can crash elasticsearch

// #[derive(Debug, Clone, Deserialize)]
// pub struct Application {
//     pub _id: String,
//     pub name: String,
//     pub indexes: Vec<String>
// }


// // Checks if app exists, also doubles as a variable if server is down
// #[derive(Debug, Clone, Deserialize)]
// pub struct Applications {
//     pub server_up: bool,
// }
// // pub app_list: Vec<Application>


// pub async fn check_server_up_app_list(app: &mut Applications) {
//     let client = EClientTesting::new("http://127.0.0.1:9200");
//     // loop {
//         sleep(Duration::from_secs(5)).await;
//         let server_up = is_server_up(&client).await;
//         println!("{server_up}");
//         *app = Applications {
//             server_up: true
//         };
//     // }
// }

// pub async fn check_if_working(app: Data::<Applications>) -> HttpResponse{
//     println!("{:#?}", app);
//     HttpResponse::Ok().json(json!({"success_maybe": app.server_up}))
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // let mut applications: Arc<Applications> = Arc::new(
    //     Applications {
    //         server_up: false,
    //     }
    // );
    // app_list: Vec::new()
    // check_server_up_app_list(&mut applications).await;
    
    // tokio::task::spawn(check_server_up_app_list(&mut applications));

    // Start server
    HttpServer::new(|| {
        App::new()
        .service(
            web::scope("/api")
                .app_data(Data::new(EClientTesting::new("http://127.0.0.1:9200")))
                // .app_data(Data::new(applications.clone()))
                .route("/app", web::post().to(initialize_new_app_id))
                .route("/apps", web::get().to(get_application_list))
                .route("/app/{app_id}", web::get().to(get_application))
                .route("/app", web::put().to(update_application))   
                .route("/app/{app_id}", web::delete().to(delete_application))
                
                .route("/index", web::post().to(_create_index))
                .route("/index/{app_id}", web::get().to(_get_index))
                .route("/index/mappings/{app_id}/{index}", web::get().to(_get_mappings))
                .route("/index/mappings", web::put().to(_update_mappings))
                .route("/index/{app_id}/{index}", web::delete().to(_delete_index))
        
                .route("/document", web::post().to(_create_document))
                .route("/document/{app_id}/{index}/{document_id}", web::get().to(_get_document))
                .route("/search", web::post().to(_post_search))
                .route("/search/{app_id}/{index}", web::get().to(_search))
                .route("/document", web::put().to(_update_document))
                .route("/document/{app_id}/{index}/{document_id}", web::delete().to(_delete_document))
        
                .service(
                    web::scope("/another_test")
                    .route("/get_keys", web::get().to(test_get_keys))
                    .route("/test_data", web::post().to(test_data))
                    // .route("/get_apps_list_test", web::get().to(check_if_working))
                )
            
            // Old
            .service(
                web::scope("/old")
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
                )
            //     .service(
            //         web::scope("/apps_without_mli")
            // )
        )
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


}