use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductFilter {
    pub id: Option<u64>,
    pub bar_code: Option<String>,
    pub name: Option<String>,
    pub category: Option<String>,
    pub image: Option<String>,
    pub base_price: Option<f64>,
    pub real_price: Option<f64>,
    pub quantity: Option<u32>
}