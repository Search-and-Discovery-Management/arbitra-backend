use crate::actions::EClientTesting;

/// Checks if the elastic server is up
pub async fn is_server_up(client: &EClientTesting) -> bool {
    match client.check_index("1").await {
        Ok(_) => return true,
        Err(_) => return false,
    }
    
}

pub fn str_or_default_if_exists_in_vec(s: &str, v: Vec<String>, default: &str) -> String {
    let st = s.to_string().to_lowercase();
    
    match v.contains(&st){
        true => st,
        false => default.to_string(),
    }
}
