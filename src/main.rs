use std::collections::HashMap;
use serde_json::{Value, json};
use actix_web::{get, post, web::{self, Data}, App, HttpServer, Responder, Result, delete, put};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
struct User {
    name: String,
    password: String,

}

#[derive(Serialize, Debug, Clone, Deserialize)]
struct UserSearch {
    name: String
}

#[derive(Serialize, Debug)]
struct Users {
    user_list: Vec<User>
}

// #[derive(Serialize, Debug)]
// struct settings {
//     number_of_shards: u32,
//     number_of_replicas: u32
// }


#[post("/api/add_user")]
async fn add_user(data: web::Json<User>, mut users: web::Data<Users>) -> Result<impl Responder> {
    // let (name, password) = data.into_inner();

    Ok(web::Json(users.user_list.clone()))
}

#[put("/api/update_user")]
async fn update_user(data: web::Json<User>, mut users: web::Data<Users>) -> Result<impl Responder> {
    // let (name, password) = data.into_inner();

    Ok(web::Json(users.user_list.clone()))
}

#[delete("/api/delete_user")]
async fn delete_user(name: web::Json<UserSearch>, mut users: web::Data<Users>) -> Result<impl Responder> {

    Ok(web::Json(users.user_list.clone()))
}

#[get("/api/user")]
async fn get_certain_user(name: web::Json<UserSearch>, users: web::Data<Users>) -> Result<impl Responder> {

    Ok(web::Json(users.user_list.clone()))
}


#[get("/api/users")]
async fn get_all_users(users: web::Data<Users>) -> Result<impl Responder> {

    Ok(web::Json(users.user_list.clone()))
}


// #[get("/api/post_to_elastic")]
// async fn post_all_users_to_elastic(users: web::Data<Users>) -> Result<impl Responder> {
//     let client = reqwest::Client::new();
//     let url = "http://127.0.0.1:9201/test1";
//     // let new_index = settings{
//     //         number_of_shards: 3,
//     //         number_of_replicas: 3
//     // };

    
//     let new_index = json!(
//         r#"{"settings" : {"index" : {"number_of_shards" : 5,"number_of_replicas" : 2}}}"#
//     );

//     let resp1 = 
//         client
//         .put(url)
//         // .json(&new_index)
//         // .header("Connection", "keep-alive")
//         .send()
//         .await;

//     let resp2= 
//         client
//         .put(url)
//         .json(&users)
//         .send()
//         .await;

//         println!("resp1 {:#?}", resp1);
//         println!("resp2 {:#?}", resp2);



//     Ok(web::Json(users.user_list.clone()))
// }

#[derive(Serialize, Debug, Clone, Deserialize)]
struct SearchTerm{
    term: String
}

// #[get("/api/search/{term}")]
// async fn find_search_term(search_term: web::Path<SearchTerm>) -> Result<impl Responder> {
//     let client = reqwest::Client::new();
//     let url = "http://127.0.0.1:9201/test1";
//     // let new_index = settings{
//     //         number_of_shards: 3,
//     //         number_of_replicas: 3
//     // };

    
//     let new_index = json!(r#"
//         {
//             "settings": {
//                 "number_of_shards": 3,
//                 "number_of_replicas": 2
//             }
//         }"#);

//     let resp1 = 
//         client
//         .post(url)
//         .json(&new_index)
//         .send()
//         .await;

//     let resp2= 
//         client
//         .post(url)
//         .json(&users)
//         .send()
//         .await;

//     Ok(web::Json(users.user_list.clone()))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = reqwest::Client::new();
    let mut x = Data::new(Users{
        user_list: vec![
            User{
                name:"Test".to_string(), 
                password: "Password".to_string()
        }]
    });
    HttpServer::new(move || {
        App::new()
            .app_data(x.clone())
            .service(get_all_users)
            .service(get_certain_user)
            .service(delete_user)
            .service(add_user)
            .service(update_user)
            .service(post_all_users_to_elastic)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}