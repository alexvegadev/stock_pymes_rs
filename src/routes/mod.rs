use actix_web::web;

use crate::handlers;


pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    // CLIENT
   
    cfg.service(handlers::client::get_client_by_id);
    cfg.route("/client", web::put().to(handlers::client::update_client));
    cfg.route("/client", web::post().to(handlers::client::create_client));
    cfg.route("/client/{id}", web::delete().to(handlers::client::remove_client));
    cfg.route("/clients", web::get().to(handlers::client::get_clients));
    cfg.route(
        "/clients/find",
        web::get().to(handlers::client::find_clients_by_filter),
    );

    // PRODUCT
    cfg.route("/product", web::get().to(handlers::product::get_products));
    cfg.route("/product", web::put().to(handlers::product::update_product));
    cfg.route("/product", web::post().to(handlers::product::create_product));
    cfg.route("/product/{id}", web::delete().to(handlers::product::remove_product));
    cfg.route("/product/find", web::get().to(handlers::product::find_products_by_filter));

    // SALE
    cfg.route("/sale", web::post().to(handlers::sale::create_sale));
    cfg.route("/sale", web::delete().to(handlers::sale::remove_sale));
    cfg.route("/sale", web::get().to(handlers::sale::get_sales));

    //OPERATION
    cfg.route("/operation", web::post().to(handlers::operation::process_operation));
}