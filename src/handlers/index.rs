use actix_web::{web::{self, Data}, HttpResponse};

use crate::actions::EClientTesting;

/// Index interfaces with application_id
/// Creating a new index accesses application_list which finds application_id of that specific index, then adds a new index to the id's list
/// 
pub async fn create_index(client: Data::<EClientTesting>) -> HttpResponse {  
    // Adds index to an application id, then creates a number of new index 

    todo!()
}

pub async fn get_index(client: Data::<EClientTesting>) -> HttpResponse {  
    // Retrieves either one or all index from an application id, returns index or 404 if not found
    // Retrieves index from an application id, returns index or 404 if not found

    todo!()
}

pub async fn get_mappings(client: Data::<EClientTesting>) -> HttpResponse {
    // Returns the mappings of an index

    todo!()
}

pub async fn update_mappings(client: Data::<EClientTesting>) -> HttpResponse {  
    // Updates the mappings of an index, which subsequently updates the rest of the sharded index with the new field

    todo!()
}


pub async fn delete_index(client: Data::<EClientTesting>) -> HttpResponse {  
    // Deletes index along with its shard, then removes itself from the application id's index list

    todo!()
}