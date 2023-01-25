use actix_web::{web::{self, Data}, Responder, Result, put};
use serde_json::Value;
use crate::EClient;

/*
JSON Data Format For Update(Might change):
    {
        index: index_name
        data: {
            id
            name
            password
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/


#[put("/api/update_data")]
pub async fn update_data_on_index(updated_data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {
    // Update data in index

    // Ok(web::Json(users.user_list.clone()))
    Ok(web::Json(updated_data.clone()))
}