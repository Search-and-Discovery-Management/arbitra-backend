use actix_web::{web::{self, Data}, HttpResponse};

use crate::actions::EClientTesting;

/// Document interfaces with index that is stored within the application id
/// Inserting a document with a new field syncs the fields with all other shards
/// 
/// All operations requires app_id and the index name
pub async fn create_document(client: Data::<EClientTesting>) -> HttpResponse {  
    // Creates a new document by getting application id, index name, check if document has new field, if yes, check dynamic mode
    // if true, update the entire index shards to accomodate the new field, then insert

    todo!()
}

pub async fn get_document(client: Data::<EClientTesting>) -> HttpResponse {  
    //

    todo!()
}

pub async fn search(client: Data::<EClientTesting>) -> HttpResponse {  
    // Searches the whole index
    todo!()
}

pub async fn update_document(client: Data::<EClientTesting>) -> HttpResponse {  

    todo!()
}

pub async fn delete_document(client: Data::<EClientTesting>) -> HttpResponse {  

    todo!()
}