#[macro_use]
extern crate serde_derive;
pub mod markers;
pub mod routes;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

pub async fn run() -> std::io::Result<()> {
    if let Err(e) = markers::update_markers().await {
        dbg!(e);
    }
    HttpServer::new(|| {
        let cors = Cors::default().allowed_origin("http://localhost:8080");
        App::new().wrap(cors).service(routes::markers::get)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
