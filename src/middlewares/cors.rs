use actix_cors::Cors;

pub fn cors() -> Cors {
    Cors::permissive()
}