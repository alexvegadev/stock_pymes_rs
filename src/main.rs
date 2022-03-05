use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

mod dto;
mod handlers;
mod db;
mod request;
mod routes;
mod utils;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_config : db::DatabaseConfig = db::DatabaseConfig::new();
    db_config.install_db();
    let pool = web::Data::new(db_config.get_pool());
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(ping)
            .configure(routes::setup_routes)
            //PRODUCTS
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

