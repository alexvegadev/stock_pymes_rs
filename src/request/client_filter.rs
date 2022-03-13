use serde::{Serialize, Deserialize};

use super::Operator;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientFilter {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub operator: Option<Operator>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientUpdate {
    pub id: Option<u32>,
    pub name: Option<String>
}
