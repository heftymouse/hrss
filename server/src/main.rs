mod api;
mod models;
mod db;
mod util;

use std::time::UNIX_EPOCH;

use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::do_thing)
    })
    .bind(("127.0.0.1", 4555))?
    .run()
    .await
}