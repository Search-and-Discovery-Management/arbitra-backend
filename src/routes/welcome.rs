use actix_web::{HttpResponse, get};

#[get("/api")]
pub async fn welcome() -> HttpResponse{
    HttpResponse::Ok().body("Welcome to the Domain Platform Services API, Documentation:\n\nhttps://github.com/AgapeKagemine/dps/blob/dev/api_contract.md")
}