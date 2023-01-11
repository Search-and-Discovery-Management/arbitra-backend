use std::sync::Mutex;

use actix_web::{get, post, web::{self, Data}, App, HttpResponse, HttpServer, Responder, Result, delete, put};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
struct User {
    name: String,
    password: String,

}

#[derive(Serialize, Debug)]
struct Users {
    user_list: Vec<User>
}

#[post("/api/add_user/{name}/{password}")]
async fn add_user(data: web::Path<(String, String)>, users: web::Data<Users>) -> Result<impl Responder> {
    let (name, password) = data.into_inner();

    Ok(web::Json(users.user_list.clone()))
}




#[put("/api/update_user/{name}/{password}")]
async fn update_user(data: web::Path<(String, String)>, users: web::Data<Users>) -> Result<impl Responder> {
    let (name, password) = data.into_inner();

    Ok(web::Json(users.user_list.clone()))
}

#[delete("/api/delete_user/{name}")]
async fn delete_user(name: web::Path<String>, users: web::Data<Users>) -> Result<impl Responder> {

    Ok(web::Json(users.user_list.clone()))
}

#[get("/api/user/{name}")]
async fn get_certain_user(name: web::Path<String>, users: web::Data<Users>) -> Result<impl Responder> {

    Ok(web::Json(users.user_list.clone()))
}


#[get("/api/users")]
async fn get_all_users(name: web::Path<String>, users: web::Data<Users>) -> Result<impl Responder> {

    Ok(web::Json(users.user_list.clone()))
}

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}