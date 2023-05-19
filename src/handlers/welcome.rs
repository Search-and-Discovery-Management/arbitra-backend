use actix_web::{HttpResponse};

pub async fn welcome() -> HttpResponse{
    HttpResponse::Ok().body("Welcome to the Domain Platform Services API, Documentation:\n\nhttps://github.com/Search-and-Discovery-Management/arbitra-backend/blob/experimental-dev-mli-2/api_contract.md")
}