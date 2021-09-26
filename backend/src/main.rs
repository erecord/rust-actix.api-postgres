mod config;
mod db;
mod handlers;
mod models;

use crate::handlers::*;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!(
        "Server started on http://{}:{}/",
        config.server.host, config.server.port
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
