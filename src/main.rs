use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use mysql::{prelude::Queryable, Opts, Pool, PooledConn};
use once_cell::sync::Lazy;
use std::sync::Mutex;

mod dto;
mod handlers;

const CREATE_TABLE_CLIENT: &str = "CREATE TABLE if not exists client(id int NOT NULL AUTO_INCREMENT, name varchar(150), PRIMARY KEY(id))";
const CREATE_TABLE_PRODUCT: &str = "CREATE TABLE if not exists product(id int NOT NULL AUTO_INCREMENT, bar_code varchar(100), name varchar(100), category varchar(50), image varchar(150), base_price decimal(18, 2), real_price decimal(18, 2),  quantity int, PRIMARY KEY(id))";
const CREATE_TABLE_SALE: &str = "CREATE TABLE if not exists sale(id int NOT NULL AUTO_INCREMENT, client_id int, product_id int, date_purchase date, PRIMARY KEY(id))";

static POOL: Lazy<Mutex<Pool>> = Lazy::new(|| {
    dotenv::dotenv().ok();
    let url = dotenv::var("DATABASE_URL").unwrap_or(String::from(""));

    Mutex::new(Pool::new(Opts::from_url(&*url).unwrap()).unwrap())
});

pub fn get_conn() -> PooledConn {
    POOL.lock().unwrap().get_conn().unwrap()
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    install_db();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(ping)
            .route("/clients", web::get().to(handlers::get_clients))
            .route("/client/{id}", web::get().to(handlers::get_client_by_id))
            .route("/client", web::post().to(handlers::create_client))
            .route(
                "/clients/find",
                web::get().to(handlers::find_clients_by_filter),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn install_db() {
    let mut conn = get_conn();
    conn.query_drop(CREATE_TABLE_CLIENT).unwrap();
    conn.query_drop(CREATE_TABLE_PRODUCT).unwrap();
    conn.query_drop(CREATE_TABLE_SALE).unwrap();
}
