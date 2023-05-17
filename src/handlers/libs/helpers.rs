use reqwest::StatusCode;

use crate::{actions::EClient, handlers::{libs::index_exists, errors::ErrorTypes}, AppConfig};

/// Checks if the elastic server is up
pub async fn is_server_up(client: &EClient) -> bool {
    client.check_index("1").await.is_ok()
}

/// Checks if 1. Server is up, 2. App and Index exists
pub async fn check_server_up_exists_app_index(app_id: &str, index: &str, client: &EClient, app_config: &AppConfig) -> Result<(), (StatusCode, ErrorTypes)>{

    if is_server_up(client).await {
        match index_exists(app_id, index, client, app_config).await {
            Ok(_) => return Ok(()),
            Err((status, err, _)) => return Err((status, err))
        }
    }

    Err((StatusCode::SERVICE_UNAVAILABLE, ErrorTypes::ServerDown))
}