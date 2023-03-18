use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewApp{
    pub app_name: String
}

#[derive(Deserialize)]
pub struct SearchApp{
    pub app_name: Option<String>
}

#[derive(Deserialize)]
pub struct GetApp{
    pub app_id: String
}

#[derive(Deserialize)]
pub struct UpdateApp{
    pub app_id: String,
    pub app_name: String
}

#[derive(Deserialize)]
pub struct DeleteApp{
    pub app_id: String
}