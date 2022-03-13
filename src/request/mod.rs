use serde::{Deserialize, Serialize};

pub mod client_filter;
pub mod update_filters;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Operator {
    AND,
    OR
}