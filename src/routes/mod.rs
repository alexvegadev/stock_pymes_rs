use actix_web::web;

use crate::handlers;


pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    // CLIENT
    cfg.service(handlers::client::get_client_by_id);
    cfg.route("/client", web::put().to(handlers::client::update_client));
    cfg.route("/clients", web::get().to(handlers::client::get_clients));
    cfg.route("/client", web::post().to(handlers::client::create_client));
    cfg.route(
        "/clients/find",
        web::get().to(handlers::client::find_clients_by_filter),
    );
    cfg.route("/client/{id}", web::delete().to(handlers::client::remove_client));

    // PRODUCT
    cfg.route("/product", web::get().to(handlers::product::get_products));
    cfg.route("/product", web::put().to(handlers::product::update_product));
}