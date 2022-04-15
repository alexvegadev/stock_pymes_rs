use std::collections::HashMap;

use mysql::PooledConn;

use crate::dto::operation::{AbstractOperation, OperationType};

use super::month_operation::SumBetweenMonthsOperation;

pub struct OperationDispatcher;

impl OperationDispatcher {
    pub fn dispatch(operation: OperationType) -> Box<dyn AbstractOperation> {
        match operation {
            OperationType::SalesBetweenMonths => Box::new(SumBetweenMonthsOperation {}),
            OperationType::DefaultOperation => Box::new(DefaultOperation {}),
        }
    }
}

struct DefaultOperation;

impl AbstractOperation for DefaultOperation {
    fn calculate(
        &self,
        conn: &mut PooledConn,
        args: HashMap<String, String>,
    ) -> HashMap<String, String> {
        let mut response: HashMap<String, String> = HashMap::new();
        response.insert("status".to_owned(), "OK".to_owned());
        response.insert("conn_id".to_owned(), format!("{}", conn.connection_id()));
        args.iter().for_each(|obj| {
            response.insert(obj.0.to_string(), obj.1.to_string());
        });
        response
    }
}
