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
    name: Option<String>
}


pub async fn get_clients() -> Result<impl Responder> {
    let mut conn = get_conn();
    let map = conn.query_map(SELECT_CLIENTS, |(id, name)| {
        Client{id: id, name: name}
    }).unwrap();
    Ok(web::Json(map))
}

pub async fn get_client_by_id(paths: web::Path<(u64,)>) -> Result<impl Responder> {
    let mut conn = get_conn();
    let query = format!("{} where id={}", SELECT_CLIENTS, paths.0);
    let result: Option<(u32, String)> =  conn.query_first(query).unwrap();
    let client_opt = result.unwrap();
    Ok(web::Json(Client{id: Some(client_opt.0.into()), name: client_opt.1}))
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