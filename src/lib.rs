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
use actix_web::{
    middleware::Logger,
    rt::{spawn, time},
    App, HttpServer,
};
use dotenv::dotenv;
use std::env;
use std::time::Duration;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::create_connection_pool();
    let conn = pool.get().expect("Failed to get connection.");
    let update_interval = env::var("UPDATE_INTERVAL")
        .expect("UPDATE_INTERVAL is not set in .env file")
        .parse::<u64>()
        .expect("UPDATE_INTERVAL must be an positive integer");
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(update_interval));
        loop {
            interval.tick().await;
            if let Err(e) = models::Marker::update(&conn).await {
                dbg!(e);
            }
        }
    });

    HttpServer::new(move || {
        let cors = Cors::default().allowed_origin("http://localhost:8080");
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .data(pool.clone())
            .service(routes::markers::get)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
