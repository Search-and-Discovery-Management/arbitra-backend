use actix_web::web;
use actix_web::{get, web::{Data}, Responder, Result};
use serde_json::{Value, json};
use crate::EClient;

/*

JSON Data Format For Get(Might change):
    {
        index: index_name
        search_term: ABC
        Search_in: (field_name)
        return_fields: {
            id
            name
            password
            ...
        }
    }
    From serde_json value, extract: 
    let x = var.get("str");
*/


#[get("/api/find_in_index")]
pub async fn search_in_index(data: web::Json<Value>, elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    // println!("{:#?}", data);
    let index = data.get("index");
    if index == None {
        println!("Fail");
        
    }
    println!("{:#?}", index.unwrap());
    // println!("{:#?}", index.unwrap());

    // let x = index.unwrap().as_str();


    let search_term: String = 
    
        match data.get("SearchTerm"){
            None => {
                println!("Search term not supplied");
                // return Ok(web::Json(data.clone()));
                "".to_string()
            },
            Some(x) => {
                println!("Search Term: {:#?}", x);
                x.to_string()
            }
        };


        let fields_to_return = 
    
            match data.get("return_fields"){
                None => {
                    println!("Search term not supplied");
                    // return Ok(web::Json(data.clone()));
                    // vec!["*"]
                    // json!({"*"})
                    "".to_string()
                },
                Some(x) => {
                    println!("Fields: {:#?}", x);
                    x.to_string()
                }
            };
        println!("fields_to_return: {:#?}", fields_to_return);


    let resp = elasticsearch_client.find_document(index.unwrap().as_str().unwrap(), Some(&search_term), None, Some(fields_to_return)).await;

    println!("{:#?}", resp);

    // let resp = elasticsearch_client
    // .search(SearchParts::Index(&[&(index.unwrap().to_string())]))
    // .body(json!({
    //     "query": {
    //         "match": {
    //             "body": &search_term
    //         }
    //     }
    // }))
    // .send()
    // .await; // missing "?"

    Ok(web::Json(data.clone()))
}


#[get("/api/get_index_list")]
async fn get_all_index(elasticsearch_client: Data::<EClient>) -> Result<impl Responder> {

    // if exists: return a list of index
    let resp = elasticsearch_client.get_all_index().await;

    Ok(web::Json(resp.clone()))

}