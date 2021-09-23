mod models;
use actix_web::{HttpServer, App, web, Responder};
use std::io;
use models::Status;

async fn status() -> impl Responder{
    web::Json(Status{status:"Ok".to_string(), number: 12})
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(status))
            
    })
    .bind("0.0.0.0:5000")?
            .run()
            .await
}
