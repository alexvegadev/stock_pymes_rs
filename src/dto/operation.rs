use std::collections::HashMap;

use mysql::PooledConn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum OperationType {
    SalesBetweenMonths,
    DefaultOperation
}

pub trait AbstractOperation {
    fn calculate(&self, conn: &mut PooledConn, args: HashMap<String, String>) -> HashMap<String, String>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OperationRequest {
    pub operation_type: OperationType,
    pub args: HashMap<String, String>
}
