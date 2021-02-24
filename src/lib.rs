#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
pub mod db;
pub mod models;
pub mod mw_api;
pub mod routes;
pub mod schema;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

use dotenv::dotenv;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::create_connection_pool();
    let conn = pool.get().unwrap();
    if let Err(e) = models::Marker::update(&conn).await {
        dbg!(e);
    }

    HttpServer::new(move || {
        let cors = Cors::default().allowed_origin("http://localhost:8080");
        App::new()
            .data(pool.clone())
            .wrap(cors)
            .service(routes::markers::get)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
