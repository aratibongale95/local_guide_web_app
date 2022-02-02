#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate actix_cors;

use actix_web::{http,App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use actix_cors::Cors;

mod db; 
mod error_handler;
mod schema;
mod model;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| 
        App::new().wrap(
            Cors::default()
                .allow_any_origin()
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::CONTENT_TYPE,
                ])
                .allowed_header(http::header::CONTENT_TYPE)
                .allowed_header("api-key")
                .max_age(3600),
        ).configure(model::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}