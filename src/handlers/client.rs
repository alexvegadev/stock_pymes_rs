use actix_web::{error, web, Result, Responder, HttpResponse, get};
use mysql::{prelude::Queryable, Pool};

use crate::{dto::{Client, ApiOk, ApiError}, request::client_filter::{ClientFilter, ClientUpdate, Operator}};

const SELECT_CLIENTS: &str = "SELECT id, name from client";
const INSERT_CLIENT: &str = "INSERT INTO client(name) VALUES(?)";
const DELETE_CLIENT: &str = "DELETE FROM client where id=?";
const UPDATE_CLIENT: &str = "UPDATE client SET {} where id=?";


pub async fn get_clients(pool: web::Data<Pool>) -> Result<impl Responder> {
    let mut conn = pool.get_conn().unwrap();
    let clients = conn.query_map(SELECT_CLIENTS, |(id, name)| {
        Client{id: id, name: name}
    }).unwrap();
    Ok(web::Json(clients))
}

#[get("/client/{id}")]
async fn get_client_by_id(pool: web::Data<Pool>, paths: web::Path<(u64,)>) -> Result<HttpResponse, error::Error> {
    let mut conn = pool.get_conn().unwrap();
    let query = format!("{} where id={}", SELECT_CLIENTS, paths.0);
    let result: Option<(u32, String)> =  conn.query_first(query).unwrap();

    if result != None {
        let client_opt = result.unwrap();
        return Ok(HttpResponse::Ok().json(Client{id: Some(client_opt.0.into()), name: client_opt.1}))
    }

    Ok(HttpResponse::NotFound().json(ApiError{ status_code: 404, error: "Client not found".to_string(), message: "The client doesn't exists".to_string() }))
}

pub async fn find_clients_by_filter(pool: web::Data<Pool>, web::Query(qry): web::Query<ClientFilter>) -> Result<impl Responder> {
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
    let mut conn = pool.get_conn().unwrap();
    let clients = conn.query_map(query, |(id, name)| {
        Client{id: id, name: name}
    }).unwrap();
    Ok(web::Json(clients))
}

pub async fn create_client(pool: web::Data<Pool>, client: web::Json<Client>) -> Result<impl Responder> {
    let mut conn = pool.get_conn().unwrap();
    let mut new_client = client.0.clone();
    let name = &client.name;
    conn.exec_drop(INSERT_CLIENT, (name,)).unwrap();
    let insert_id = conn.last_insert_id();
    new_client.id = Some(insert_id);
    Ok(web::Json(new_client))
}

pub async fn remove_client(pool: web::Data<Pool>, paths: web::Path<(u64,)>) -> Result<impl Responder> {
    let mut conn = pool.get_conn().unwrap();
    let id = paths.0;
    conn.exec_drop(DELETE_CLIENT, (id,)).unwrap();
    Ok(web::Json(ApiOk{ message: "client removed!".to_owned() }))
}

pub async fn update_client(pool: web::Data<Pool>, client: web::Json<ClientUpdate>) -> Result<HttpResponse, error::Error> {
    let client_update = client.0.clone();
    if client_update.id == None {
        return Ok(HttpResponse::BadRequest().json(ApiError{ message: "You need to insert the id".to_string(), status_code: 400, error: "Invalid data to update".to_string() }));
    }
    let mut query = String::from("");
    if client_update.name != None {
        let fmt = format!("name='{}'", client_update.name.unwrap());
        query.push_str(fmt.as_str());
    }
    let mut conn = pool.get_conn().unwrap();
    let orig_query = UPDATE_CLIENT.replace("{}", query.as_str());
    conn.exec_drop(orig_query, (client_update.id.unwrap(),)).unwrap();
    Ok(HttpResponse::Ok().json(client.0))
}