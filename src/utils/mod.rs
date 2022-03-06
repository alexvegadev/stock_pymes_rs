use std::fmt::{Debug, Display};

use crate::request::Operator;

fn comma_set(qry: String) -> String {
    if qry.len() > 0 {
        ",".to_string()
    } else {
        "".to_string()
    }
}

pub fn push_if_not_none<T>(t: Option<T>, name: &str, query: &mut String)
where
    T: Debug + PartialEq + Display, 
{

    if t != None {
        let mut val_fmt: String = String::from("%val%");
        let val_str = format!("{}", t.unwrap());
        
        if val_str.chars().nth(0).unwrap().is_numeric() {
            val_fmt = val_fmt.replace("%val%", &val_str);
        } else {
            val_fmt = val_fmt.replace("%val%", format!("'{}'", val_str).as_str());
        }
        let fmt = format!("{}{}={}", comma_set(query.to_string()), name, val_fmt.to_string());
        query.push_str(fmt.as_str());
    }
}


fn append_if_zero(qry: String, op: String) -> String {
    if qry.len() > 0 {
        
        format!(" {} ", op)
    } else {
        "".to_string()
    }
}

pub fn push_where_filter<T>(t: Option<T>, name: &str, query: &mut String, operator: Operator)
where
    T: Debug + PartialEq + Display, 
{
    
    if t != None {
        let mut val_fmt: String = String::from("%val%");
        let val_str = format!("{}", t.unwrap());
        let op = format!("{:#?}", operator);
        
        if val_str.chars().nth(0).unwrap().is_numeric() {
            val_fmt = val_fmt.replace("%val%", &val_str);
        } else {
            val_fmt = val_fmt.replace("%val%", format!("'{}'", val_str).as_str());
        }
        let fmt = format!("{}{}={}", append_if_zero(query.to_string(), op), name, val_fmt.to_string());
        query.push_str(fmt.as_str());
    }
}