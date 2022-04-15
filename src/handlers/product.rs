use actix_web::{error, web, HttpResponse, Responder, Result};
use mysql::{prelude::Queryable, Pool};

use crate::{
    dto::{ApiError, ApiOk, Product},
    request::{update_filters::ProductFilter, Operator},
    utils::{
        push_if_not_none, push_where_filter, query::{PageBuilder, PageResult}
    },
};

const SELECT_PRODUCT: &str =
    "SELECT id, bar_code, name, category, image, base_price, real_price, quantity from product";
const INSERT_PRODUCT: &str = "INSERT INTO product(bar_code, name, category, image, base_price, real_price, quantity) VALUES(?, ?, ?, ?, ?, ?, ?)";
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

pub async fn create_product(
    pool: web::Data<Pool>,
    product: web::Json<Product>,
) -> Result<impl Responder> {
    let mut conn = pool.get_conn().unwrap();
    let mut new_product = product.0.clone();
    let bar_code = &product.bar_code;
    let name = &product.name;
    let category = &product.category;
    let image = &product.image;
    let base_price = &product.base_price;
    let real_price = &product.real_price;
    let quantity = &product.quantity;
    conn.exec_drop(
        INSERT_PRODUCT,
        (
            bar_code, name, category, image, base_price, real_price, quantity,
        ),
    )
    .unwrap();
    let insert_id = conn.last_insert_id();
    new_product.id = Some(insert_id);
    Ok(web::Json(new_product))
}

pub async fn remove_product(
    pool: web::Data<Pool>,
    paths: web::Path<(u64,)>,
) -> Result<HttpResponse, error::Error> {
    let mut conn = pool.get_conn().unwrap();
    let id = paths.0;
    let res = conn.exec_drop(DELETE_PRODUCT, (id,));
    if res.is_ok() {
        res.unwrap();
        return Ok(HttpResponse::Created().json(ApiOk {
            message: "product removed!".to_owned(),
        }));
    }
    Ok(HttpResponse::NotFound().json(ApiError {
        status_code: 404,
        error: "Entity not found!".to_string(),
        message: "product not found!".to_owned(),
    }))
}

pub async fn find_products_by_filter(
    pool: web::Data<Pool>,
    web::Query(pager): web::Query<PageBuilder>,
    web::Query(qry): web::Query<ProductFilter>,
) -> Result<impl Responder> {
    let mut query = SELECT_PRODUCT.to_owned();
    let mut cond = String::new();
    let op = if qry.operator == None {
        Operator::AND
    } else {
        qry.operator.unwrap()
    };

    push_where_filter(qry.id, "id", &mut cond, op);
    push_where_filter(qry.name, "name", &mut cond, op);
    push_where_filter(qry.category, "category", &mut cond, op);
    println!("{}", cond);
    if cond.len() > 0 {
        query += " where ";
        query += cond.as_str();
    }
    let mut conn = pool.get_conn().unwrap();

    let products = conn
        .query_map(
            pager.build(query.as_str()),
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
    let pages = pager.calculate_count(&mut conn, "product");
    Ok(web::Json(PageResult{page_count: pages, pager, results: products}))
}
