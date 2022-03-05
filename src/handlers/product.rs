use actix_web::{error, get, web, HttpResponse, Responder, Result};
use mysql::{prelude::Queryable, Pool};

use crate::{
    dto::{ApiError, Product},
    request::update_filters::ProductFilter,
    utils::{push_if_not_none}
};


const SELECT_PRODUCT: &str = "SELECT * from product";
const INSERT_PRODUCT: &str = "INSERT INTO product(name) VALUES(?)";
const DELETE_PRODUCT: &str = "DELETE FROM product where id=?";
const UPDATE_PRODUCT: &str = "UPDATE product SET {} where id=?";

pub async fn get_products(pool: web::Data<Pool>) -> Result<impl Responder> {
    let mut conn = pool.get_conn().unwrap();
    let products = conn
        .query_map(
            SELECT_PRODUCT,
            |(id, bar_code, name, category, image, base_price, real_price, quantity)| Product {
                id,
                bar_code,
                name,
                category,
                image,
                base_price,
                real_price,
                quantity,
            },
        )
        .unwrap();
    Ok(web::Json(products))
}

pub async fn update_product(
    pool: web::Data<Pool>,
    product: web::Json<ProductFilter>,
) -> Result<HttpResponse, error::Error> {
    let prod_update = product.0.clone();
    if prod_update.id == None {
        return Ok(HttpResponse::BadRequest().json(ApiError {
            message: "You need to insert the id".to_string(),
            status_code: 400,
            error: "Invalid data to update".to_string(),
        }));
    }
    let mut query = String::from("");
    push_if_not_none(prod_update.name, "name", &mut query);
    push_if_not_none(prod_update.bar_code, "bar_code", &mut query);
    push_if_not_none(prod_update.base_price, "base_price", &mut query);
    push_if_not_none(prod_update.category, "category", &mut query);
    push_if_not_none(prod_update.image, "image", &mut query);
    push_if_not_none(prod_update.quantity, "quantity", &mut query);
    push_if_not_none(prod_update.real_price, "real_price", &mut query);
    
    let mut conn = pool.get_conn().unwrap();
    let orig_query = UPDATE_PRODUCT.replace("{}", query.as_str());
    conn.exec_drop(orig_query, (prod_update.id.unwrap(),))
        .unwrap();
    Ok(HttpResponse::Ok().json(product.0))
}

