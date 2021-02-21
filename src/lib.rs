#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate jsonapi;
pub mod routes;
use actix_web::{App, HttpServer};

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::markers::get))
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
