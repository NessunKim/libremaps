pub mod markers;

use actix_web::{get, Error, HttpResponse};

#[get("/")]
async fn get() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hi!"))
}
