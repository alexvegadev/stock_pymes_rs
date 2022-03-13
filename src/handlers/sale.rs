use actix_web::{web, Result, Responder, HttpResponse, error};
use mysql::{prelude::Queryable, Pool};

use crate::{
    dto::{ApiOk, Sale, ApiError}
};

const SELECT_SALES: &str = "SELECT * from sale";
const INSERT_SALES: &str = "INSERT INTO sale(client_id, product_id, qty, date_purchase) VALUES(?,?,?,?)";
const DELETE_SALES: &str = "DELETE FROM sale where id=?";
//const UPDATE_CLIENT: &str = "UPDATE client SET {} where id=?";


pub async fn get_sales(pool: web::Data<Pool>) -> Result<impl Responder> {
    let mut conn = pool.get_conn().unwrap();
    let sales = conn.query_map(SELECT_SALES, |(id,client_id, product_id, quantity, date_purchase)| {
        Sale{id, client_id, product_id, quantity, date_purchase}
    }).unwrap();
    Ok(web::Json(sales))
}

pub async fn create_sale(pool: web::Data<Pool>, client: web::Json<Sale>) -> Result<impl Responder> {
    let mut conn = pool.get_conn().unwrap();
    let mut new_sale = client.0.clone();
    if new_sale.date_purchase == None {
        let now = mysql_common::chrono::Local::now();
        new_sale.date_purchase = Some(now.naive_utc());
    }
    conn.exec_drop(INSERT_SALES, (new_sale.client_id, new_sale.product_id, new_sale.quantity, new_sale.date_purchase, )).unwrap();
    let insert_id = conn.last_insert_id();
    new_sale.id = Some(insert_id);
    Ok(web::Json(new_sale))
}

pub async fn remove_sale(pool: web::Data<Pool>, paths: web::Path<(u64,)>) -> Result<HttpResponse, error::Error> {
    let mut conn = pool.get_conn().unwrap();
    let id = paths.0;
    let res = conn.exec_drop(DELETE_SALES, (id,));
    match res {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(ApiOk{ message: "sale removed!".to_owned() }))
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiError{ message: "Error removing".to_owned(), status_code: 505, error: e.to_string() }))
        },
    }
}

