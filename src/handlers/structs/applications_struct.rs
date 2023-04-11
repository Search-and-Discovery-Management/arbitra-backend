use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequiredAppName{
    pub app_name: String
}

#[derive(Deserialize)]
pub struct OptionalAppName{
    pub app_name: Option<String>
}

#[derive(Deserialize)]
pub struct RequiredAppID{
    pub app_id: String
}

#[derive(Deserialize)]
pub struct UpdateApp{
    pub app_id: String,
    pub app_name: String
}