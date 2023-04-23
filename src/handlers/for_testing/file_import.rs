use std::{io::{Read}, collections::HashMap};
use actix_multipart::{form::{MultipartForm, tempfile::TempFile}};
use actix_web::{web, HttpResponse};
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{handlers::{structs::index_struct::RequiredIndex, libs::{bulk_create, check_server_up_exists_app_index}, errors::FileErrorTypes}, actions::EClient, MAX_FILE_SIZE};

#[derive(MultipartForm)]
pub struct Upload {
    file: TempFile,
}

/// Inserts data from file into an existing index (Accepts json, csv, and tsv)
pub async fn create_by_file(app_index: web::Path<RequiredIndex>, f: MultipartForm<Upload>, client: web::Data<EClient>) -> HttpResponse {
    println!("Route Create Document by File");

    match check_server_up_exists_app_index(&app_index.app_id, &app_index.index, &client).await{
        Ok(_) => (),
        Err((status, err)) => return HttpResponse::build(status).json(json!({"error": err.to_string()})),
    }

    let file_name = f.file.file_name.clone().unwrap();
    let file_size = f.file.size;
    let mut file = f.file.file.reopen().unwrap();
    
    let file_name_split: Vec<&str> = file_name.split('.').collect();
    let extension = file_name_split.last().unwrap().to_ascii_lowercase();
    println!("file name: {:#?}", file_name);
    println!("file size: {:#?}", file_size);

    // Check the file size
    if file_size > MAX_FILE_SIZE {
        return HttpResponse::PayloadTooLarge().json(json!({"error": FileErrorTypes::FileTooLarge(file_size, MAX_FILE_SIZE).to_string()}))
    }

    // Check extensions, only allow json, csv, and tsv
    // TODO: File from CSV and TSV
    
    if extension.eq(&"json".to_string()) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let data: Result<Vec<Value>, _> = serde_json::from_str(&contents);
        match data{
            Ok(x) => bulk_create(&app_index.app_id, &app_index.index, &x, &client).await,
            Err(_) => HttpResponse::BadRequest().json(json!({"error": FileErrorTypes::InvalidFile("json".to_string()).to_string()}))
        }
    
    } else if extension.eq(&"csv".to_string()) || extension.eq(&"tsv".to_string()) {

        let sep = if extension.eq(&"csv".to_string()){
            b','
        } else {
            b'\t'
        };

        let mut contents = csv::ReaderBuilder::new()
            .delimiter(sep)
            .from_reader(file);

        let mut data: Vec<Value> = vec![];
        for (curr, i) in contents.deserialize().enumerate() {
            match i {
                Ok(val) => {
                    // Turn into Hashmap type before converting into value
                    let z: HashMap<String, Value> = val;
                    data.push(serde_json::to_value(z).unwrap())
                },
                Err(_) => return HttpResponse::build(StatusCode::BAD_REQUEST).json(json!({"error": FileErrorTypes::InvalidLine(curr).to_string()})),
            }
        }

        return bulk_create(&app_index.app_id, &app_index.index, &data, &client).await;
    } else {
        return HttpResponse::BadRequest().json(json!({"error": FileErrorTypes::InvalidFileExtension(".json, .csv, .tsv".to_string()).to_string()}))
    }
}