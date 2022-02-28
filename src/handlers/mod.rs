use actix_web::{web, Result, Responder};
use mysql::prelude::Queryable;
use serde::{Serialize, Deserialize};

use crate::{get_conn, dto::Client};


const SELECT_CLIENTS: &str = "SELECT id, name from client";
const INSERT_CLIENT: &str = "insert into client(name) values(?)";

/* 
    ************************ CLIENT IMPL 
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientFilter {
    id: Option<u32>,
    name: Option<String>,
    operator: Option<Operator>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    AND,
    OR
}


pub async fn get_clients() -> Result<impl Responder> {
    let mut conn = get_conn();
    let clients = conn.query_map(SELECT_CLIENTS, |(id, name)| {
        Client{id: id, name: name}
    }).unwrap();
    Ok(web::Json(clients))
}

pub async fn get_client_by_id(paths: web::Path<(u64,)>) -> Result<impl Responder> {
    let mut conn = get_conn();
    let query = format!("{} where id={}", SELECT_CLIENTS, paths.0);
    let result: Option<(u32, String)> =  conn.query_first(query).unwrap();
    let client_opt = result.unwrap();
    Ok(web::Json(Client{id: Some(client_opt.0.into()), name: client_opt.1}))
}

pub async fn find_clients_by_filter(web::Query(qry): web::Query<ClientFilter>) -> Result<impl Responder> {
    let mut query = SELECT_CLIENTS.to_owned();
    if qry.id != None || qry.name != None {
        query +=  " where ";
    }
    if qry.id != None {
        let fmt = format!("id={}", qry.id.unwrap());
        query.push_str(fmt.as_str());
    }
    if qry.name != None {
        let fmt = format!("name like '%{}%'", qry.name.unwrap());
        if query.contains("id=") {
            let op = if qry.operator != None && qry.operator.unwrap() == Operator::OR {" OR "} else {" AND "};
            
            query.push_str((op.to_owned() + &fmt).as_str());
        } else {
            query.push_str(fmt.as_str());
        }
    }
    println!("{}", query);
    let mut conn = get_conn();
    let clients = conn.query_map(query, |(id, name)| {
        Client{id: id, name: name}
    }).unwrap();
    Ok(web::Json(clients))
}

pub async fn create_client(client: web::Json<Client>) -> Result<impl Responder> {
    let mut conn = get_conn();
    let mut new_client = client.0.clone();
    let name = &client.name;
    conn.exec_drop(INSERT_CLIENT, (name,)).unwrap();
    let insert_id = conn.last_insert_id();
    new_client.id = Some(insert_id);
    Ok(web::Json(new_client))
}