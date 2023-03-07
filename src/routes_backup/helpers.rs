use actix_web::HttpResponse;
use serde_json::{Value, json};


/// Checks if string is supplied, and is type of str, if yes, return str, else return HttpResponse with error
/// 
/// If bool, return either true or false as string
#[allow(dead_code)]
pub fn required_check_string(index: Option<&Value>, field: &str) -> Result<String, HttpResponse>{
    match index {
        Some(val) => {
            if val.is_string(){
                return Ok(val.as_str().unwrap().to_string())
            } else if val.is_boolean(){
                if val.as_bool().unwrap(){
                    return Ok("true".to_string());
                } else {
                    return Ok("false".to_string());
                }
            } else { 
                return 
                    Err(HttpResponse::BadRequest().json(
                    json!({
                        "error_message": field.to_owned() + " must be in string"
                    })))
            }
        },
        None => return 
            Err(HttpResponse::BadRequest().json(
                json!({
                    "error_message": field.to_owned() + " not supplied"
                }))
            )
    };
}

/// Checks if string is supplied, then check if type is string, if either is false, return None
#[allow(dead_code)]
pub fn optional_check_string(value_str: Option<&Value>) -> Option<String>{
    match value_str {
        Some(val) => {
            if val.is_string(){
                Some(val.as_str().unwrap().to_string())
            } else{ 
            return None
        }
        },
        None => None
    }
}

#[allow(dead_code)]
pub fn required_check_value(value: Option<&Value>, field: &str) -> Result<Value, HttpResponse>{
    match value{
        Some(val) => {
            if val.is_object(){
                Ok(val.clone())
            } else { 
                return 
                    Err(HttpResponse::BadRequest().json(
                        json!({
                            "error_message": field.to_owned() + " must be in value"
                        }))
                    )
            }
        },
        None => return 
            Err(HttpResponse::BadRequest().json(
                json!({
                    "error_message": field.to_owned() + " not supplied"
                })
            ))
    }
}

#[allow(dead_code)]
pub fn optional_check_value(value: Option<&Value>) -> Option<Value>{
    match value{
        Some(val) => {
            if val.is_object(){
                Some(val.clone())
            } else { 
                return None
            }
        },
        None => None
    }
}

#[allow(dead_code)]
pub fn optional_check_bool(value_bool: Option<&Value>) -> Option<bool>{
    match value_bool{
        Some(val) => {
            if val.is_boolean(){
                val.as_bool()
            } else if val.is_string() {
                let bool = val.as_str().unwrap().to_lowercase();
                    if bool.eq("true") || bool.eq("false"){
                        return Some(bool.parse::<bool>().unwrap())
                    }
                    return None
                } else { 
                    return None
            }   
        },
        None => None
    }
}

#[allow(dead_code)]
pub fn optional_check_number(value_num: Option<&Value>) -> Option<i64>{
    match value_num{
        Some(val) => {
            if val.is_number(){
                val.as_i64()
            } else if val.is_string() {
                let number = val.as_str().unwrap().parse::<i64>();
                    if number.is_ok(){
                        return Some(number.unwrap())
                    }
                    return None
                } 
                else { 
                    return None
                }   
            },
        None => None
    }
}

#[allow(dead_code)]
pub fn str_or_default_if_exists_in_vec(s: &str, v: Vec<String>, default: &str) -> String {
    let st = s.to_string().to_lowercase();
    
    match v.contains(&st){
        true => st,
        false => default.to_string(),
    }
}