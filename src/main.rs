use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use config::config::Config;

mod dto;
mod handlers;
mod db;
mod request;
mod routes;
mod utils;
mod config;


#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let conf = Config::new("./config/config.yaml");
    let db_url = conf.get_conf("config")["DATABASE_URL"].as_str().unwrap();
    let install_db: bool = conf.get_conf("config")["AUTO_INSTALL_DB"].as_bool().unwrap();
    let host: &str = conf.get_conf("config")["HOST"].as_str().unwrap();
    let port: i64 = conf.get_conf("config")["PORT"].as_i64().unwrap();
    let db_config : db::DatabaseConfig = db::DatabaseConfig::new(db_url);
    if install_db {
        println!("Installing DB...");
        db_config.install_db();
    }
    let pool = web::Data::new(db_config.get_pool());
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(ping)
            .configure(routes::setup_routes)
            //PRODUCTS
    })
    .bind((host, (port as u16)))?
    .run()
    .await
}

