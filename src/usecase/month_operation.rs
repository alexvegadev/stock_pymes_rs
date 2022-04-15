use std::collections::HashMap;

use mysql::{params, prelude::Queryable};

use crate::dto::operation::AbstractOperation;

const OPERATION: &str = "
SELECT 
client_id,
sum(p.real_price * s.quantity) as sales_sum
from sales as s
inner join product as p
on p.id = s.product_id
group by client_id
where s.client_id=:client_id AND
s.date_purchase between :date_start AND :date_end;
";

struct OperationResponse {
    client_id: i64,
    sales_sum: f64,
}

pub struct SumBetweenMonthsOperation;

impl AbstractOperation for SumBetweenMonthsOperation {
    fn calculate(
        &self,
        conn: &mut mysql::PooledConn,
        args: HashMap<String, String>,
    ) -> HashMap<String, String> {
        let cid = args.get("client_id").unwrap();
        let date_start = args.get("date_start").unwrap();
        let date_end = args.get("date_end").unwrap();
        let mut response: HashMap<String, String> = HashMap::new();
        let cid_num: i64 = cid.parse().unwrap();
        let res = conn
            .exec_first(
                OPERATION,
                params! {
                    "client_id" => cid_num,
                    "date_start" => date_start,
                    "date_end" => date_end,
                },
            )
            .map(|opt| {
                opt.map(|(client_id, sales_sum)| OperationResponse {
                    client_id: client_id,
                    sales_sum: sales_sum,
                })
            })
            .unwrap();
        match res {
            Some(resp) => {
                response.insert("client_id".to_string(), resp.client_id.to_string());
                response.insert("total_sales_price".to_string(), resp.sales_sum.to_string());
            }
            None => {}
        }
        response
    }
}
