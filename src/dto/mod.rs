use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    pub id: Option<u64>,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Product {
    pub id: Option<u64>,
    pub bar_code: Option<String>,
    pub name: String,
    pub category: String,
    pub image: Option<String>,
    pub base_price: f64,
    pub real_price: f64,
    pub quantity: u32
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sale {
    pub id: Option<u64>,
    pub client_id: Option<i64>,
    pub product_id: u64,
    pub quantity: u32,
    pub date_purchase: Option<mysql_common::chrono::NaiveDateTime>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub message: String,
    pub status_code: u16,
    pub error: String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ApiOk {
    pub message: String,
}

