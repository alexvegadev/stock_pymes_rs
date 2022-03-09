use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct QueryBuilder {
    queries: Vec<Query>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Query {
    pub name: String,
    pub logic_op: String,
    pub comparison_op: ComparisonOperator,
    pub value: String
}

#[derive(PartialEq, Deserialize, Serialize, Clone)]
pub enum ComparisonOperator {
    Contains,
    Equals,
    In,
    StartsWith,
    EndsWith
}

impl QueryBuilder {
    pub fn new() -> Self {
        return Self{queries: vec!()}
    }
    pub fn from(qry: Vec<Query>) -> Self {
        return Self{queries: qry}
    }
}

impl QueryTrait<'_> for QueryBuilder {

    fn build(&self) -> String {
        let mut query: String = String::new();
        for i in &self.queries {
            if query.len() > 0 {
                query.push_str(format!(" {} ", i.logic_op.as_str()).as_str());
            }
            query.push_str(&(i.name.to_owned() + &" ".to_owned()));
            //VALUE
            let format: &str = if query.chars().nth(0).unwrap().is_numeric() {"%s"} else {"'%s'"};
            //
            if i.comparison_op == ComparisonOperator::Equals {
                query.push_str("=");
                query.push_str(&format.replace("%s", &i.value));
            } else if i.comparison_op == ComparisonOperator::In {
                let spl = i.value.split(",");
                let mapping = spl.map(| spl | {
                    format.replace("%s", spl.trim())
                });
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
        self.queries.push(field);
    }

    
}

pub trait QueryTrait<'a> {
    fn add(&mut self, field: Query);
    fn build(&self) -> String;
}
