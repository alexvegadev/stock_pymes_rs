use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use config::config::Config;
use log::info;

mod dto;
mod handlers;
mod db;
mod request;
mod routes;
mod utils;
mod config;
mod secret;


#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let conf = Config::new("./config/");
    let db_url: &str = conf.get_str("config", "DATABASE_URL");
    let install_db: bool = conf.get_bool("config", "AUTO_INSTALL_DB");
    let host: &str = conf.get_str("config", "HOST");
    let port_res = conf.get_conf("config")["PORT"].as_i64();
    let db_config : db::DatabaseConfig = db::DatabaseConfig::new(db_url);
    if install_db {
        info!("Installing DB...");
        db_config.install_db();
    }
    let port: u16;
    if port_res == None {
        port = std::env::var("PORT").unwrap().parse::<u16>().unwrap_or(8080);
    } else {
        port = port_res.unwrap() as u16;
    }
    let pool = web::Data::new(db_config.get_pool());
    HttpServer::new(move || {
        App::new()
            .default_service(web::to(|| HttpResponse::NotFound()))
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(ping)
            .configure(routes::setup_routes)
            //PRODUCTS
    })
    .bind((host, port))?
    .run()
    .await
}

