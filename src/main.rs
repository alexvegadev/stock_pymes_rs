use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

mod dto;
mod handlers;
mod db;



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
            .service(handlers::client::get_client_by_id)
            .route("/client", web::put().to(handlers::client::update_client))
            .route("/clients", web::get().to(handlers::client::get_clients))
            .route("/client", web::post().to(handlers::client::create_client))
            .route(
                "/clients/find",
                web::get().to(handlers::client::find_clients_by_filter),
            )
            .route("/client/{id}", web::delete().to(handlers::client::remove_client))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

