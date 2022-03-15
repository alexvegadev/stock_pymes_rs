use mysql::{prelude::Queryable, Pool, PooledConn};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct PageBuilder {
    pub page: u32,
    pub items: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PageResult<T> {
    pub results: Vec<T>,
    pub pager: PageBuilder,
    pub page_count: u32
}

struct Count {
    pub count: u32
}

impl PageBuilder {
    fn calc_offset(&self) -> u32 {
        (self.page - 1) * self.items
    }

    pub fn build(&self, sel: &str) -> String {
        format!("{} LIMIT {},{}", sel, self.calc_offset(), self.items)
    }

    pub fn calculate_count(&self, con: &mut PooledConn, table_name: &str) -> u32 {
        let count = con
            .query_first(format!("SELECT COUNT(1) FROM {}", table_name))
            .map(|row| {
                row.map(|count| Count {
                    count
                })
            });
        match count.unwrap() {
            Some(c) => {
                let cnt: f32 = c.count as f32;
                let itm: f32 = self.items as f32;
                let pages = cnt / itm;
                
                pages.ceil() as u32
            },
            None => 0,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct QueryBuilder {
    filters: Vec<Query>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Query {
    pub name: String,
    pub logic_op: String,
    pub comparison_op: ComparisonOperator,
    pub value: String,
}

#[derive(PartialEq, Deserialize, Serialize, Clone)]
pub enum ComparisonOperator {
    Contains,
    Equals,
    In,
    StartsWith,
    EndsWith,
}

impl QueryBuilder {
    pub fn new() -> Self {
        return Self { filters: vec![] };
    }
    pub fn from(qry: Vec<Query>) -> Self {
        return Self { filters: qry };
    }
}

impl QueryTrait<'_> for QueryBuilder {
    fn build(&self) -> String {
        let mut query: String = String::new();
        for i in &self.filters {
            if query.len() > 0 {
                query.push_str(format!(" {} ", i.logic_op.as_str()).as_str());
            }
            query.push_str(&(i.name.to_owned() + &" ".to_owned()));
            //VALUE
            let format: &str = if query.chars().nth(0).unwrap().is_numeric() {
                "%s"
            } else {
                "'%s'"
            };
            //
            if i.comparison_op == ComparisonOperator::Equals {
                query.push_str("=");
                query.push_str(&format.replace("%s", &i.value));
            } else if i.comparison_op == ComparisonOperator::In {
                let spl = i.value.split(",");
                let mapping = spl.map(|spl| format.replace("%s", spl.trim()));
                query.push_str("IN(");
                for m in mapping {
                    query.push_str(&(m + ","));
                }
                let start = query.len() - 1;
                let stop = query.len();
                query.replace_range(start..stop, "");
                query.push_str(") ");
            }
        }
        query
    }

    fn add(self: &mut QueryBuilder, field: Query) {
        self.filters.push(field);
    }
}

pub trait QueryTrait<'a> {
    fn add(&mut self, field: Query);
    fn build(&self) -> String;
}
